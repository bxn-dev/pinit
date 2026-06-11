use std::{fs, path::Path, process::Command, collections::HashMap};

fn base(path: &Path) -> Result<(), String> {
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
        Err(err) => Err(err.to_string()),
        Ok(_) => Ok(()),
    }
}

fn create_template(path: &Path, template: &str) -> Result<(), String> {
    match Command::new("git")
        .arg("clone")
        .arg(template)
        .current_dir(&path)
        .status()
    {
        Err(err) => Err(err.to_string()),
        Ok(_) => Ok(()),
    }
}

pub fn create_project(path: &Path, template: Option<&str>) -> Result<(), String> {

        let templates = HashMap::from([
            ("base_cli", ""),
                
            ]);
    
        if let Some(template) = template {
            create_template(&path, template)
        }
        else {
            base(&path)
        }
    }
}
