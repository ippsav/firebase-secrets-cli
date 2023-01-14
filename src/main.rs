use clap::Parser;



pub mod cli;

use cli::*;


fn main() {
    let cli = Cli::parse();

    match cli.commands{
        Commands::Set(options) => {
            dbg!(options);
        },
    }
}
