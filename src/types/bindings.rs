use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Bindings {
    pub module_bindings: HashMap<String, ModuleBindings>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ModuleBindings {
    pub version_bindings: HashMap<String, VersionBindings>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct VersionBindings {
    pub entries: Vec<Binding>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Binding {
    pub internal_path: PathBuf,
    pub external_path: PathBuf,
}
