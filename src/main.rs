use clap::Parser;

fn main() {
    let cli = rayql::cli::Cli::parse();

    match &cli.command {
        Some(rayql::cli::Commands::Generate { filename }) => {
            rayql::cli::commands::generate::handle_generate(filename.clone());
        }
        None => (),
    }
}
