use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
    process::Command,
};

struct Template {
    name: &'static str,
    url: &'static str,
}

const RUST_TEMPLATES: &[Template] = &[
    Template {
        name: "base_cli",
        url: "https://github.com/bxn-dev/rust_cli_basic_template.git",
    },
    // TODO: Add base_lib template
    // Template {
    //     name: "base_lib",
    //     url: "",
    // },
];

pub fn create_project(path: &Path, template: Option<&str>) -> Result<(), String> {
    if !path.exists() {
        if let Err(err) = fs::create_dir_all(path) {
            return Err(err.to_string());
        }
    }

    if let Some(template) = template {
        match RUST_TEMPLATES.iter().find(|t| t.name == template) {
            Some(template) => create_template(path, template.url),
            None => return Err("Template not found".to_string()),
        }
    } else {
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

fn create_template(path: &Path, template: &str) -> Result<(), String> {
    let clone_output = Command::new("git")
        .arg("clone")
        .arg(template)
        .arg(path)
        .arg("--depth")
        .arg("1")
        .output();

    match clone_output {
        Err(err) => return Err(err.to_string()),
        Ok(output) => {
            if !output.status.success() {
                return Err(String::from_utf8_lossy(&output.stderr).to_string());
            }
            println!("Template cloned");
        }
    }

    let git_path = match path.join(".git").canonicalize() {
        Err(err) => return Err(err.to_string()),
        Ok(path) => path,
    };

    // Remove the .git directory, this exists, because we just cloned, so .git should exists
    match fs::remove_dir_all(git_path) {
        Err(err) => return Err(err.to_string()),
        Ok(_) => {
            println!("Git directory deleted");
        }
    };

    // Remove .github, if it exists
    let gh_workflow_path = path.join(".github");
    if gh_workflow_path.exists() {
        match gh_workflow_path.canonicalize() {
            Ok(path) => match fs::remove_dir_all(path) {
                Err(err) => return Err(err.to_string()),
                Ok(_) => {
                    println!("Workflow directory deleted");
                }
            },
            Err(err) => return Err(err.to_string()),
        };
    }

    match personalize(path) {
        Ok(_) => {
            println!("Cargo.toml modified");
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}

fn personalize(path: &Path) -> Result<(), String> {
    let toml_path = path.join("Cargo.toml");
    let project_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| "Could not extract project name from path".to_string())?;

    let toml_content = match fs::read_to_string(&toml_path) {
        Err(err) => return Err(err.to_string()),
        Ok(content) => content,
    };

    let mut toml_file = match File::create(&toml_path) {
        Err(err) => return Err(err.to_string()),
        Ok(file) => file,
    };

    let mut writer = BufWriter::new(&mut toml_file);

    for line in toml_content.lines() {
        if line.contains("name =") {
            writeln!(writer, "name = \"{}\"", project_name).map_err(|e| e.to_string())?;
        } else {
            writeln!(writer, "{}", line).map_err(|e| e.to_string())?;
        }
    }

    match writer.flush() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
