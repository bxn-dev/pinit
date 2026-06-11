use std::path::Path;

pub fn create_project(path: &Path, template: Option<&str>) {
    if let Some(t) = template {
        println!("Creating Python project in {:?} with template {}", path, t);
    } else {
        println!(
            "Creating Python project in {:?} with default template",
            path
        );
    }
}
