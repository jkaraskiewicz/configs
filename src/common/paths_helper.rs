use std::path::{Path, PathBuf};

use anyhow::Result;
use commons::utils::path_util::get_home_dir_path;

pub fn get_path_suffix_in_home(path: &Path) -> Result<PathBuf> {
    let home_path = get_home_dir_path()?;
    let suffix = path.strip_prefix(&home_path)?;
    Ok(suffix.to_path_buf())
}

pub fn convert_to_internal_path(path: &Path) -> Result<PathBuf> {
    let mut iter = path.components();
    let first_component = iter.next().unwrap().as_os_str().to_str().unwrap();
    if let Some(prefix) = first_component.strip_prefix(".") {
        let new_prefix = format!("dot_{}", prefix);
        let new_suffix = path.strip_prefix(first_component)?;
        Ok(PathBuf::from(new_prefix).join(new_suffix))
    } else {
        Ok(path.to_path_buf())
    }
}

pub fn convert_to_external_path(path: &Path) -> Result<PathBuf> {
    let mut iter = path.components();
    let first_component = iter.next().unwrap().as_os_str().to_str().unwrap();
    if let Some(prefix) = first_component.strip_prefix("dot_") {
        let new_prefix = format!(".{}", prefix);
        let new_suffix = path.strip_prefix(first_component)?;
        Ok(PathBuf::from(new_prefix).join(new_suffix))
    } else {
        Ok(path.to_path_buf())
    }
}
