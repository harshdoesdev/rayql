use clap::Parser;

#[tokio::main]
async fn main() {
    let cli = rayql::cli::Cli::parse();

    match cli.command {
        Some(rayql::cli::Commands::Generate) => rayql::sql::generate(),
        Some(rayql::cli::Commands::Db(db_args)) => match db_args.command {
            Some(rayql::cli::DbCommands::Push) => rayql::db::push().await,
            None => (),
        },
        None => (),
    }
}
