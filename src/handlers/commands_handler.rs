use std::fs::{self, create_dir_all};

use anyhow::Result;
use versions::{Module, Version};

use crate::types::bindings::VersionBindings;

use super::{
    bindings_handler::{read_version_bindings, unbind_all, update_version_bindings},
    repository_handler::get_current_repository,
    workspace_handler::unlink_version,
};

pub fn add_version(name: &str, module: &Module) -> Result<Module> {
    let mut module = module.to_owned();
    let current_version = &module.current_version;

    let mut current_version_bindings = VersionBindings::default();

    if let Some(current_version) = current_version {
        current_version_bindings = read_version_bindings(current_version)?;
    }

    let new_version = module.add_version(name)?;
    update_version_bindings(&new_version, |_| current_version_bindings.to_owned())?;

    Ok(module)
}

pub fn remove_version(version: &Version) -> Result<()> {
    let repository = get_current_repository()?;
    let mut module = repository.get_module(&version.module.module_name)?;
    let module_path = repository.root_path.join(&module.directory);

    let current_version = module.to_owned().current_version;
    let is_current = current_version.map(|el| el.name).unwrap_or_default() == version.name;

    if is_current {
        unlink_version(version)?;
        unbind_all(version)?;
        module.remove_version(&version.name)?;
        fs::remove_dir_all(&module_path)?;
        fs::create_dir_all(&module_path)?;
    } else {
        unbind_all(version)?;
        module.remove_version(&version.name)?;
    }
    Ok(())
}

pub fn add_module(name: &str) -> Result<Module> {
    let repository = get_current_repository()?;
    let module_path = repository.root_path.join(name);

    create_dir_all(module_path)?;
    let module = repository.add_module(name, name)?;

    let current_version = repository.get_module(name)?.force_current_version()?;
    update_version_bindings(&current_version, |bindings| bindings.to_owned())?;

    Ok(module)
}

pub fn add_module_with_version(module_name: &str, version_name: &str) -> Result<()> {
    let mut module = add_module(module_name)?;
    let current_version = module.current_version.to_owned().unwrap();
    module = add_version(version_name, &module)?;
    module.select_version(version_name)?;
    remove_version(&current_version)?;
    Ok(())
}

pub fn remove_module(module: &Module) -> Result<()> {
    let repository = get_current_repository()?;
    let module_path = repository.root_path.join(&module.directory);

    let current_version = module.current_version.to_owned();

    if let Some(current_version) = current_version {
        unlink_version(&current_version)?;
    };

    for version in &module.versions {
        unbind_all(version)?;
    }

    repository.remove_module(module)?;
    fs::remove_dir_all(&module_path)?;

    Ok(())
}
