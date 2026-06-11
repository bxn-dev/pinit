use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod langs;

#[derive(Parser, Debug)]
#[command(about = "A tool to initialize projects of various kinds.", version)]
struct Cli {
    #[command(subcommand)]
    command: Langs,
}

#[derive(Subcommand, Debug)]
enum Langs {
    /// initialize a new rust project
    Rust {
        /// the template that should be used
        #[arg(short, long)]
        template: Option<String>,

        /// Path used to initialize the project
        path: PathBuf,
    },

    Python {
        /// the template that should be used
        #[arg(short, long)]
        template: Option<String>,

        /// Path used to initialize the project
        path: PathBuf,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    match cli.command {
        Langs::Rust { template, path } => {
            if let Err(e) = langs::rust::create_project(path.as_path(), template.as_deref()) {
                eprintln!("Error creating Rust project: {}", e);
            }
        }
        Langs::Python { template, path } => {
            langs::python::create_project(path.as_path(), template.as_deref());
        }
    }
}
