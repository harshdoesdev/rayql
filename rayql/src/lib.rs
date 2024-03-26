use clap::{command, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Print,
    Db(DbArgs),
}

#[derive(Args)]
#[command(flatten_help = true)]
pub struct DbArgs {
    #[command(subcommand)]
    pub command: Option<DbCommands>,
}

#[derive(Subcommand)]
pub enum DbCommands {
    Push,
}
