use std::path::Path;

use anyhow::Result;
use commons::{
    traits::collections::ContainsPredicate,
    utils::file_util::{read_file, write_file},
};
use versions::{Module, Version};

use crate::{
    common::{
        constants,
        paths_helper::{convert_to_internal_path, get_path_suffix_in_home},
    },
    types::bindings::{Binding, Bindings, ModuleBindings, VersionBindings},
};

pub fn is_path_bound(version: &Version, path: &Path) -> Result<bool> {
    let version_bindings = read_version_bindings(version)?;
    let contains = version_bindings
        .entries
        .contains(|el| el.external_path.starts_with(path));
    Ok(contains)
}

pub fn bind_path(version: &Version, path: &Path) -> Result<Binding> {
    let mut suffixed_path = match get_path_suffix_in_home(path) {
        Ok(path_suffix) => path_suffix,
        Err(_) => path.to_path_buf(),
    };
    suffixed_path = convert_to_internal_path(&suffixed_path)?;
    let new_binding = Binding {
        internal_path: suffixed_path.to_path_buf(),
        external_path: path.to_path_buf(),
    };
    update_bindings(version, |version_binding| {
        let mut entries = version_binding.entries.to_owned();
        entries.push(new_binding.to_owned());
        VersionBindings { entries }
    })?;
    Ok(new_binding)
}

pub fn unbind_path(version: &Version, path: &Path) -> Result<Binding> {
    let current_binding = read_version_bindings(version)?
        .entries
        .iter()
        .find(|el| el.external_path == path)
        .unwrap()
        .to_owned();
    update_bindings(version, |version_binding| {
        let mut entries = version_binding.entries.to_owned();
        entries.retain(|binding| binding.external_path != path);
        VersionBindings { entries }
    })?;
    Ok(current_binding)
}

pub fn unbind_all(version: &Version) -> Result<()> {
    update_bindings(version, |_| VersionBindings {
        entries: Vec::new(),
    })
}

fn update_bindings(
    version: &Version,
    updater: impl Fn(&VersionBindings) -> VersionBindings,
) -> Result<()> {
    let mut bindings = read_bindings(&version.module.repository_path).unwrap_or_else(|_| {
        let default_bindings = Bindings::default();
        write_bindings(&version.module.repository_path, &default_bindings).unwrap();
        default_bindings
    });
    let mut module_binding = bindings
        .module_bindings
        .get(&version.module.module_name)
        .map(|e| e.to_owned())
        .unwrap_or_default();
    let version_binding = module_binding
        .version_bindings
        .get(&version.name)
        .map(|e| e.to_owned())
        .unwrap_or_default();
    module_binding
        .version_bindings
        .insert(version.name.to_string(), updater(&version_binding));
    bindings
        .module_bindings
        .insert(version.module.module_name.to_string(), module_binding);
    write_bindings(&version.module.repository_path, &bindings)?;
    Ok(())
}

pub fn read_bindings(repository_path: &Path) -> Result<Bindings> {
    let bindings_path = repository_path.join(constants::BINDINGS_CONFIG_FILE);

    let content = read_file(&bindings_path)?;
    let bindings: Bindings = serde_yml::from_str(&content)?;
    Ok(bindings)
}

pub fn read_module_bindings(module: &Module) -> Result<ModuleBindings> {
    let bindings = read_bindings(&module.repository_ptr.repository_path)?;
    Ok(bindings
        .module_bindings
        .get(&module.name)
        .map(|e| e.to_owned())
        .unwrap_or_default())
}

pub fn read_version_bindings(version: &Version) -> Result<VersionBindings> {
    let bindings = read_bindings(&version.module.repository_path)?;
    let module_bindings = bindings
        .module_bindings
        .get(&version.module.module_name)
        .map(|e| e.to_owned())
        .unwrap_or_default();
    Ok(module_bindings
        .version_bindings
        .get(&version.name)
        .map(|e| e.to_owned())
        .unwrap_or_default())
}

pub fn write_bindings(repository_path: &Path, bindings: &Bindings) -> Result<()> {
    let bindings_path = repository_path.join(constants::BINDINGS_CONFIG_FILE);

    let content = serde_yml::to_string(bindings)?;
    write_file(&bindings_path, &content)?;
    Ok(())
}
