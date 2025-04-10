use std::{error::Error, fmt, path::PathBuf};

#[derive(Debug)]
pub enum ConfigsError {
    ConfigNotFound,
    ModuleNotSelected,
    ModuleAlreadyExists(String),
    PathAlreadyBound(PathBuf),
    PathNotBound(PathBuf),
    IncorrectLink(PathBuf),
    CannotLink(PathBuf),
}

impl fmt::Display for ConfigsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigsError::ConfigNotFound => write!(f, "Config file not found."),
            ConfigsError::ModuleNotSelected => write!(f, "No module is selected."),
            ConfigsError::ModuleAlreadyExists(name) => write!(f, "Module {} already exists.", name),
            ConfigsError::PathAlreadyBound(path) => {
                write!(
                    f,
                    "Path {} or its ancestor is already bound.",
                    path.to_str().unwrap()
                )
            }
            ConfigsError::PathNotBound(path) => {
                write!(f, "Path {} is not bound.", path.to_str().unwrap())
            }
            ConfigsError::IncorrectLink(path) => {
                write!(f, "Incorrect link for {}.", path.to_str().unwrap())
            }
            ConfigsError::CannotLink(path) => {
                write!(f, "Cannot link path {}.", path.to_str().unwrap())
            }
        }
    }
}

impl Error for ConfigsError {}
