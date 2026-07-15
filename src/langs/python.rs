use std::path::Path;

pub fn create_project(path: &Path, template: Option<&str>) -> Result<(), String> {
    let template = template.unwrap_or("base_de_l_assiette");
    crate::templates::create_project("python", path, template)
}
