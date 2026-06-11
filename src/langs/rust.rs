use std::{fs, path::Path, process::Command};

fn init(path: &Path) -> Result<(), String> {
    if !path.exists() {
        if let Err(fehler) = fs::create_dir_all(&path) {
            println!("Error creating the project dir: {}", fehler);
        }
    }
    match Command::new("cargo")
        .arg("init")
        .arg("--bin")
        .current_dir(&path)
        .status()
    {
        Err(fehler) => Err(fehler.to_string()),
        Ok(_) => Ok(()),
    }
}

pub fn create_project(path: &Path, template: Option<&str>) -> Result<(), String> {
    if let Some(t) = template {
        println!("Using Rust template: {}", t);
    } else {
        println!("Using default Rust template");
    }
    init(path)
}
