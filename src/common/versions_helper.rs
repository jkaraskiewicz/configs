use std::path::PathBuf;

use versions::Version;

pub fn get_module_path(version: &Version) -> PathBuf {
    version
        .module
        .repository_path
        .join(&version.module.module_dir)
}
