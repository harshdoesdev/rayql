use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = rayql::cli::Cli::parse();

    match cli.command {
        Some(rayql::cli::Commands::Print) => rayql::sql::print_schema(),
        Some(rayql::cli::Commands::Db(db_args)) => match db_args.command {
            Some(rayql::cli::DbCommands::Push) => (),
            None => (),
        },
        None => (),
    }
}
