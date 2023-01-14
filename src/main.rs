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
            if let Some(secret) = options.secret {
                builder.add_secret(secret)?;
            };
            if let Some(path) = options.path {
                let file = File::open(path).unwrap();
                builder.from_source(file)?;
            }
            let firebase_interface = builder.build();
        }
    };

    Ok(())
}
