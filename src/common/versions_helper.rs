use std::path::PathBuf;

use versions::{Module, Version};

pub fn get_module_path(version: &Version) -> PathBuf {
    version
        .module
        .repository_path
        .join(&version.module.module_dir)
}

pub fn get_version_from_name(name: &str, module: &Module) -> Version {
    module
        .versions
        .iter()
        .find(|el| el.name == name)
        .unwrap()
        .to_owned()
}
