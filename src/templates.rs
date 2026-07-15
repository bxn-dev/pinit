use serde::Deserialize;
use std::{
    collections::HashMap,
    fs,
    path::{Component, Path, PathBuf},
};

#[derive(Debug, Deserialize)]
struct Template {
    name: String,
    #[expect(
        dead_code,
        reason = "Template metadata is loaded for validation and future listing"
    )]
    description: Option<String>,
    variables: Option<HashMap<String, String>>,
    directories: Option<Vec<TemplateDirectory>>,
    files: Vec<TemplateFile>,
}

#[derive(Debug, Deserialize)]
struct TemplateDirectory {
    path: String,
}

#[derive(Debug, Deserialize)]
struct TemplateFile {
    path: String,
    content: String,
}

pub fn create_project(language: &str, path: &Path, template_name: &str) -> Result<(), String> {
    if path.exists()
        && path
            .read_dir()
            .map_err(|err| err.to_string())?
            .next()
            .is_some()
    {
        return Err(format!(
            "Target directory '{}' already exists and is not empty",
            path.display()
        ));
    }

    fs::create_dir_all(path).map_err(|err| err.to_string())?;

    let template_path = template_path(language, template_name);
    let template_content = fs::read_to_string(&template_path).map_err(|err| {
        format!(
            "Could not read template '{}': {}",
            template_path.display(),
            err
        )
    })?;
    let template = toml::from_str::<Template>(&template_content).map_err(|err| {
        format!(
            "Could not parse template '{}': {}",
            template_path.display(),
            err
        )
    })?;

    if template.name != template_name {
        return Err(format!(
            "Template file '{}' declares name '{}' instead of '{}'",
            template_path.display(),
            template.name,
            template_name
        ));
    }

    let variables = variables(path, language, template.variables.as_ref());

    if let Some(directories) = template.directories {
        for directory in directories {
            let relative_path = render(&directory.path, &variables);
            let destination = safe_destination(path, &relative_path)?;
            fs::create_dir_all(destination).map_err(|err| err.to_string())?;
        }
    }

    for file in template.files {
        let relative_path = render(&file.path, &variables);
        let destination = safe_destination(path, &relative_path)?;
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|err| err.to_string())?;
        }

        let content = render(&file.content, &variables);
        fs::write(destination, content).map_err(|err| err.to_string())?;
    }

    println!(
        "Created {} project from '{}' template",
        language, template_name
    );
    Ok(())
}

fn template_path(language: &str, template_name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("templates")
        .join(language)
        .join(format!("{}.toml", template_name))
}

fn variables(
    project_path: &Path,
    language: &str,
    template_variables: Option<&HashMap<String, String>>,
) -> HashMap<String, String> {
    let project_name = project_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project")
        .to_string();
    let identifier = sanitize_identifier(&project_name);

    let mut variables = HashMap::from([
        ("project_name".to_string(), project_name),
        ("crate_name".to_string(), identifier.clone()),
        ("package_name".to_string(), identifier),
        ("language".to_string(), language.to_string()),
    ]);

    if let Some(template_variables) = template_variables {
        for (key, value) in template_variables {
            variables
                .entry(key.clone())
                .or_insert_with(|| value.clone());
        }
    }

    variables
}

fn render(input: &str, variables: &HashMap<String, String>) -> String {
    let mut rendered = input.to_string();
    for (key, value) in variables {
        rendered = rendered.replace(&format!("{{{{{}}}}}", key), value);
    }
    rendered
}

fn safe_destination(root: &Path, relative_path: &str) -> Result<PathBuf, String> {
    let path = Path::new(relative_path);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir | Component::Prefix(_)))
    {
        return Err(format!("Template path '{}' is not allowed", relative_path));
    }

    Ok(root.join(path))
}

fn sanitize_identifier(name: &str) -> String {
    let mut identifier = String::with_capacity(name.len());
    for character in name.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            identifier.push(character.to_ascii_lowercase());
        } else {
            identifier.push('_');
        }
    }

    let trimmed = identifier.trim_matches('_');
    if trimmed.is_empty() {
        "project".to_string()
    } else {
        trimmed.to_string()
    }
}
