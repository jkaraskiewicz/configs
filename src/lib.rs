use std::{
    fs::{self},
    path::Path,
    str::from_utf8,
};

use anyhow::Result;
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use colored::Colorize;
use common::{
    colors_helper::Colorized,
    versions_helper::{get_module_path, get_version_from_name},
};
use commons::utils::shell_util::current_shell;
use handlers::{
    bindings_handler::{bind_path, is_path_bound, read_version_bindings, unbind_path},
    commands_handler::{
        add_module, add_module_with_version, add_version, remove_module, remove_version,
    },
    repository_handler::get_current_repository,
    workspace_handler::{link_binding, link_version, unlink_binding, unlink_version},
};
use path_absolutize::Absolutize;
use types::{
    bindings::VersionBindings,
    cli::{Cli, Command},
    errors::ConfigsError,
};
use versions::VersionsCli;

pub mod common;
pub mod handlers;
pub mod types;

pub fn execute() -> Result<String> {
    let cli = Cli::parse();
    let command = cli.command;
    match command {
        Command::Init => handle_init(),
        Command::Add { module, config } => handle_add(&module, &config),
        Command::Remove { module, config } => handle_remove(&module, &config),
        Command::Select { module, config } => handle_select(&module, &config),
        Command::Deselect => handle_deselect(),
        Command::Current => handle_current(),
        Command::Show => handle_show(),
        Command::Link { path } => handle_link(&path),
        Command::Unlink { path } => handle_unlink(&path),
        Command::Completions => handle_completions(),
    }
}

fn handle_init() -> Result<String> {
    let result = VersionsCli::new().init()?;
    Ok(result)
}

fn handle_completions() -> Result<String> {
    let mut buf = Vec::new();
    generate(current_shell(), &mut Cli::command(), "configs", &mut buf);
    Ok(from_utf8(buf.as_slice()).unwrap().to_string())
}

fn handle_add(module: &str, config: &Option<String>) -> Result<String> {
    let repository = get_current_repository()?;
    let module_preexisted = repository.get_module(module).is_ok();

    match config {
        Some(config) => {
            if module_preexisted {
                add_version(config, &repository.get_module(module).unwrap())?;
            } else {
                add_module_with_version(module, config)?;
            }
        }
        None => {
            if module_preexisted {
                return Err(ConfigsError::ModuleAlreadyExists(module.to_string()).into());
            } else {
                add_module(module)?;
            }
        }
    }

    let module_str = module.bold().underline();
    let config_str = match config {
        Some(config) => config.bold().underline(),
        None => repository
            .get_module(module)?
            .force_current_version()?
            .name
            .bold()
            .underline(),
    };

    if module_preexisted {
        Ok(format!(
            "Added config {} to module {}.",
            config_str, module_str
        ))
    } else {
        Ok(format!(
            "Added module {} with config {}.",
            module_str, config_str
        ))
    }
}

fn handle_remove(module: &str, config: &Option<String>) -> Result<String> {
    let repository = get_current_repository()?;
    let module = repository.get_module(module)?;

    if let Some(config) = config {
        let version = get_version_from_name(config, &module);
        remove_version(&version)?;
        Ok(format!(
            "Removed config {} from module {}.",
            config.bold().underline(),
            module.name.bold().underline()
        ))
    } else {
        remove_module(&module)?;
        Ok(format!(
            "Removed module {}.",
            module.name.bold().underline()
        ))
    }
}

fn handle_select(module: &str, config: &str) -> Result<String> {
    let repository = get_current_repository()?;

    let mut repo_module = repository.get_module(module)?;
    repo_module = repository.select_module(&Some(repo_module))?.unwrap();

    let mut diff_bindings = VersionBindings::default();
    if let Some(current_version) = &repo_module.current_version {
        diff_bindings = read_version_bindings(current_version)?;
        unlink_version(current_version)?;
    }

    let version = repo_module.select_version(config)?;

    link_version(&version, Some(diff_bindings))?;

    Ok(format!(
        "Selected module {} with config {}.",
        module.bold().underline(),
        config.bold().underline()
    ))
}

fn handle_deselect() -> Result<String> {
    let repository = get_current_repository()?;
    if let Some(current_module) = &repository.current_module()? {
        if let Some(current_version) = &current_module.current_version {
            let mut new_module = current_module.to_owned();
            new_module.deselect_version()?;
            repository.select_module(&None)?;

            unlink_version(current_version)?;

            let dir_path = repository.root_path.join(&current_module.directory);
            fs::remove_dir_all(&dir_path)?;
            fs::create_dir_all(&dir_path)?;
        }
    }
    Ok("Deselected current module and config.".to_string())
}

fn handle_current() -> Result<String> {
    let repository = get_current_repository()?;
    let current_module = repository.current_module()?;
    if let Some(current_module) = &current_module {
        if let Some(current_version) = &current_module.current_version {
            Ok(format!(
                "Module: {}, config: {}",
                current_module.name.bold().underline(),
                current_version.name.bold().underline()
            ))
        } else {
            Ok(format!(
                "Module: {}, No current config",
                current_module.name.bold().underline()
            ))
        }
    } else {
        Ok("No current module, No current config".to_string())
    }
}

fn handle_show() -> Result<String> {
    let repository = get_current_repository()?;
    let mut result: Vec<String> = Vec::new();

    for module in &repository.list_modules()? {
        if let Some(current_module) = &repository.current_module()? {
            if current_module.name == module.name {
                result.push(module.name.bold().underline().to_string());
            } else {
                result.push(module.name.to_string());
            }
        } else {
            result.push(module.name.to_string());
        }
        for (index, version) in module.versions.iter().cloned().enumerate() {
            let special_char = if index == 0 {
                "└──".dimmed()
            } else {
                "───".dimmed()
            };
            if let Some(current_version) = &module.current_version {
                if current_version.name == version.name {
                    result.push(format!(
                        "{} {}",
                        special_char,
                        version.name.bold().underline()
                    ));
                } else {
                    result.push(format!("{} {}", special_char, version.name.underline()));
                }
            } else {
                result.push(format!("{} {}", special_char, version.name.underline()));
            };
            let bindings = read_version_bindings(&version)?;
            for binding in bindings.entries {
                result.push(format!(
                    "    {} -> {}",
                    binding.internal_path.colorize(true),
                    binding.external_path.colorize(false)
                ));
            }
        }
    }

    Ok(result.join("\n"))
}

fn handle_link(path: &Path) -> Result<String> {
    let repository = get_current_repository()?;
    let current_version = repository.force_current_module()?.force_current_version()?;
    let path = path.absolutize().unwrap().to_path_buf();

    let already_bound = is_path_bound(&current_version, &path)?;
    if already_bound {
        return Err(ConfigsError::PathAlreadyBound(path).into());
    };

    let binding = bind_path(&current_version, &path)?;
    link_binding(&binding, &get_module_path(&current_version))?;
    Ok(format!("Linked path: {}", &path.to_str().unwrap()))
}

fn handle_unlink(path: &Path) -> Result<String> {
    let repository = get_current_repository()?;
    let current_version = repository.force_current_module()?.force_current_version()?;
    let path = path.absolutize().unwrap().to_path_buf();

    let already_bound = is_path_bound(&current_version, &path)?;
    if !already_bound {
        return Err(ConfigsError::PathNotBound(path).into());
    };

    let binding = unbind_path(&current_version, &path)?;
    unlink_binding(&binding, &get_module_path(&current_version))?;

    let internal_path = binding.internal_path;
    if internal_path.is_file() {
        fs::remove_file(&internal_path)?;
    } else if internal_path.is_dir() {
        fs::remove_dir_all(&internal_path)?;
    }

    Ok(format!("Unlinked path: {}", &path.to_str().unwrap()))
}
