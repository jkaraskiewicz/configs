use std::{fs, path::Path};

use anyhow::Result;
use commons::utils::file_util::copy;
use itertools::Itertools;
use symlink::{remove_symlink_auto, symlink_auto};
use versions::Version;

use crate::{
    common::versions_helper::get_module_path,
    types::{
        bindings::{Binding, VersionBindings},
        errors::ConfigsError,
    },
};

use super::bindings_handler::read_version_bindings;

pub fn unlink_version(version: &Version) -> Result<()> {
    let bindings = read_version_bindings(version)?;
    unlink_bindings(&bindings.entries, version)
}

pub fn link_version(version: &Version, diff_bindings: Option<VersionBindings>) -> Result<()> {
    let bindings = read_version_bindings(version)?.entries.to_owned();
    link_bindings(&bindings, version)?;

    let diff_bindings = diff_bindings.unwrap_or_default();
    let removed_bindings = diff_bindings
        .entries
        .iter()
        .filter(|el| !bindings.contains(el))
        .collect_vec();

    for binding in removed_bindings {
        let external_path = binding.external_path.to_path_buf();
        let internal_path = version
            .module
            .repository_path
            .join(&version.module.module_dir)
            .join(&binding.internal_path);
        if external_path.is_symlink() && internal_path.exists() {
            remove_symlink_auto(&external_path)?;
            fs::rename(internal_path, external_path)?;
        }
    }
    Ok(())
}

fn unlink_bindings(bindings: &[Binding], version: &Version) -> Result<()> {
    let module_dir_path = get_module_path(version);
    for binding in bindings {
        unlink_binding(binding, &module_dir_path)?;
    }
    Ok(())
}

fn link_bindings(bindings: &[Binding], version: &Version) -> Result<()> {
    let module_dir_path = get_module_path(version);
    for binding in bindings {
        link_binding(binding, &module_dir_path)?;
    }
    Ok(())
}

pub fn link_binding(binding: &Binding, module_dir_path: &Path) -> Result<()> {
    let external_path = binding.external_path.to_path_buf();
    let internal_path = module_dir_path.join(&binding.internal_path);

    let is_structure_correct =
        (external_path.exists() && !external_path.is_symlink()) || internal_path.exists();
    if !is_structure_correct {
        return Err(ConfigsError::CannotLink(external_path.to_path_buf()).into());
    }

    if !internal_path.exists() {
        copy(&external_path, &internal_path)?;
    };

    if external_path.is_file() {
        fs::remove_file(&external_path)?;
    } else if external_path.is_dir() {
        fs::remove_dir_all(&external_path)?;
    }

    symlink_auto(internal_path, external_path)?;

    Ok(())
}

pub fn unlink_binding(binding: &Binding, module_dir_path: &Path) -> Result<()> {
    let external_path = binding.external_path.to_path_buf();
    let internal_path = module_dir_path.join(&binding.internal_path);

    let is_link_correct = external_path.exists()
        && internal_path.exists()
        && external_path.is_symlink()
        && !internal_path.is_symlink();
    if !is_link_correct {
        return Err(ConfigsError::IncorrectLink(external_path.to_path_buf()).into());
    };

    remove_symlink_auto(&external_path)?;

    copy(&internal_path, &external_path)?;

    Ok(())
}
