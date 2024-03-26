use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = rayql::Cli::parse();

    match cli.command {
        Some(rayql::Commands::Print) => {
            print_schema();

            Ok(())
        }
        Some(rayql::Commands::Db(db_args)) => match db_args.command {
            Some(rayql::DbCommands::Push) => Ok(()),
            None => Ok(()),
        },
        None => Ok(()),
    }
}

pub fn print_schema() {
    let current_dir = std::env::current_dir().expect("Failed to read current dir.");
    let file_path = current_dir.join("schema.rayql");

    let code = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    let schema = match rayql_engine::Schema::parse(&code) {
        Ok(schema) => schema,
        Err(err) => {
            eprintln!(
                "{}",
                rayql_engine::error::generate_error_message(&err, &code)
            );
            std::process::exit(1);
        }
    };

    let sql_statements = match schema.to_sql() {
        Ok(stmts) => stmts,
        Err(err) => {
            eprintln!(
                "{}",
                rayql_engine::error::pretty_to_sql_error_message(err, &code)
            );
            std::process::exit(1);
        }
    };

    let output = sql_statements.join("\n\n");

    println!("{}", output);
}
