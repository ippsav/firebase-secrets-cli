use std::fs::File;

use clap::Parser;
use lib::firebase::{BuilderError, FirebaseInterfaceBuilder};

pub mod cli;

use cli::*;

#[tokio::main]
async fn main() -> Result<(), BuilderError> {
    let cli = Cli::parse();

    match cli.commands {
        Commands::Set(options) => {
            let mut builder = FirebaseInterfaceBuilder::builder();
            builder.set_alias(options.alias);
            if options.secret.is_none() && options.path.is_none() {
                println!("firebase-secrets-cli must have a source for keys and values, run firebase-secrets-cli --help for more info");
                return Ok(());
            }
            if let Some(secret) = options.secret {
                if let Err(err) = builder.add_secret(secret) {
                    println!("{}", err);
                    return Err(err);
                };
            };
            if let Some(path) = options.path {
                let file = match File::open(path) {
                    Ok(v) => v,
                    Err(err) => {
                        println!("error opening file, {}", err.to_string());
                        return Ok(());
                    }
                };
                if let Err(err) = builder.from_source(file) {
                    println!("{}", err);
                    return Err(err);
                };
            }
            let firebase_interface = builder.build()?;

            if let Err(err) = firebase_interface.set_secrets().await {
                println!("{err}");
            }
        }
    };

    Ok(())
}
