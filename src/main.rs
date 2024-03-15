fn main() {
    // Extracting filename from CLI arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: rayql parse <filename>");
        std::process::exit(1);
    }
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
        Ok(schema) => println!("{:#?}", schema),
        Err(err) => eprintln!("{}", rayql::error::generate_error_message(&err, &code)),
    };
}
