use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Set secrets in your Google Secret Manager from a file or a key value pair (KEY=VALUE)")]
    Set(Set),
}

#[derive(Args, Debug)]
pub struct Set {
    #[arg(short, long, help = "Set a secret in firebase (KEY=VALUE)")]
    pub secret: Option<String>,
    #[arg(short, long, help = "Path to a file as a source of secrets")]
    pub path: Option<String>,
    #[arg(
        short,
        long,
        help = "Set your project alias (default, dev, staging, your_project)"
    )]
    pub alias: String,
}
