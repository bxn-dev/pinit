use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod langs;
mod templates;

#[derive(Parser, Debug)]
#[command(
    name = "Entremetier",
    about = "A side-dish chef for preparing project foundations from TOML templates.",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Langs,
}

#[derive(Subcommand, Debug)]
enum Langs {
    /// Initialize a new Rust project.
    Rust {
        /// The TOML template that should be used.
        #[arg(short, long)]
        template: Option<String>,

        /// Path used to initialize the project.
        path: PathBuf,
    },

    /// Initialize a new Python project.
    Python {
        /// The TOML template that should be used.
        #[arg(short, long)]
        template: Option<String>,

        /// Path used to initialize the project.
        path: PathBuf,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    let result = match cli.command {
        Langs::Rust { template, path } => {
            langs::rust::create_project(path.as_path(), template.as_deref())
        }
        Langs::Python { template, path } => {
            langs::python::create_project(path.as_path(), template.as_deref())
        }
    };

    if let Err(error) = result {
        eprintln!("Error creating project: {}", error);
        std::process::exit(1);
    }
}
