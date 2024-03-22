use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = rayql::cli::Cli::parse();

    match cli.command {
        Some(rayql::cli::Commands::Print) => {
            rayql::sql::print_schema();

            Ok(())
        }
        Some(rayql::cli::Commands::Db(db_args)) => match db_args.command {
            Some(rayql::cli::DbCommands::Push) => Ok(()),
            None => Ok(()),
        },
        None => Ok(()),
    }
}
