fn main() {
    // Extracting filename from CLI arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: rayql parse <filename>");
        std::process::exit(1);
    }
    let sub_command = args[1].as_str();
    let filename = &args[2];
    let current_dir = std::env::current_dir().expect("Failed to read current dir.");

    // Read content from file
    let code = match std::fs::read_to_string(current_dir.join(filename)) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    };

    // Parsing schema
    match rayql::Schema::parse(&code) {
        Ok(schema) => match sub_command {
            "parse" => println!("{:#?}", schema),
            "generate" => {
                let sql_statements = schema.to_sql();
                let output = sql_statements
                    .iter()
                    .map(|(model_name, sql_statement)| {
                        format!(
                            "-- CREATE TABLE FOR MODEL `{}`\n\n{}\n",
                            model_name, sql_statement
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

                let output_dir = current_dir.join("migrations");

                if !output_dir.exists() {
                    std::fs::create_dir(&output_dir)
                        .expect("Could not create migrations directory.");
                }

                let output_file = output_dir.join("000-migrations.sql");

                std::fs::write(output_file, output).expect("Could not write SQL migrations file.");
            }
            _ => eprintln!("Unknown command {sub_command}"),
        },
        Err(err) => eprintln!("{}", rayql::error::generate_error_message(&err, &code)),
    };
}
