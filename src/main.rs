use std::fs::File;

use clap::Parser;
use lib::firebase::{BuilderError, FirebaseInterfaceBuilder};

pub mod cli;

use cli::*;

fn main() -> Result<(), BuilderError> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Set(options) => {
            let mut builder = FirebaseInterfaceBuilder::builder();
            if options.secret.is_none() && options.path.is_none() {
                println!("firebase-secrets-cli must have a source for keys and values, run firebase-secrets-cli --help for more info");
                return Ok(());
            }
            if let Some(secret) = options.secret {
                if let Err(err) = builder.add_secret(secret) {
                    match err {
                        BuilderError::Io(err) => println!("{:?}", err),
                        BuilderError::InvalidSecretFormat(err) => println!("{err}"),
                    }
                };
            };
            if let Some(path) = options.path {
                let file = File::open(path).unwrap();
                if let Err(err) = builder.from_source(file) {
                    match err {
                        BuilderError::Io(err) => println!("{:?}", err),
                        BuilderError::InvalidSecretFormat(err) => println!("{err}"),
                    }
                };
            }
            let firebase_interface = builder.build();

            if let Err(err) = firebase_interface.set_secrets(options.alias) {
                println!("{err}");
            }
        }
    };

    Ok(())
}
