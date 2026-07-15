use std::{fs, path::Path, process::Command};

pub fn create_project(path: &Path, template: Option<&str>) -> Result<(), String> {
    if let Some(template) = template {
        crate::templates::create_project("rust", path, template)
    } else {
        fs::create_dir_all(path).map_err(|err| err.to_string())?;
        println!("Creating base binary project via cargo init");
        base(path)
    }
}

fn base(path: &Path) -> Result<(), String> {
    let result = Command::new("cargo")
        .arg("init")
        .arg("--bin")
        .current_dir(path)
        .output();

    match result {
        Err(err) => Err(err.to_string()),
        Ok(output) => {
            if !output.status.success() {
                return Err(String::from_utf8_lossy(&output.stderr).to_string());
            }
            Ok(())
        }
    }
}
