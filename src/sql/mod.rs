mod function;
pub mod to_sql;

pub mod error;

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

    let schema = match rayql::Schema::parse(&code) {
        Ok(schema) => schema,
        Err(err) => {
            eprintln!("{}", rayql::error::generate_error_message(&err, &code));
            std::process::exit(1);
        }
    };

    let sql_statements = match schema.to_sql() {
        Ok(stmts) => stmts,
        Err(err) => {
            eprintln!("{}", rayql::error::pretty_to_sql_error_message(&err, &code));
            std::process::exit(1);
        }
    };

    let output = sql_statements.join("\n\n");

    println!("{}", output);
}
