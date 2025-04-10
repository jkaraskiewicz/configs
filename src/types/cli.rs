use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "Manage tool configs easily")]
pub struct Cli {
    /// Command
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone, PartialEq)]
#[clap(rename_all = "lower_case")]
pub enum Command {
    /// Initialize configs
    Init,
    /// Add module/config
    Add {
        /// Module name
        #[arg()]
        module: String,
        /// Config name
        #[arg()]
        config: Option<String>,
    },
    /// Remove module/config
    Remove {
        /// Module name
        #[arg()]
        module: String,
        /// Config name
        #[arg()]
        config: Option<String>,
    },
    /// Select module/config
    Select {
        /// Module name
        #[arg()]
        module: String,
        /// Config name
        #[arg()]
        config: String,
    },
    /// Current module and config
    Current,
    /// Show current state (modules, configs, links)
    Describe,
    /// Link a path to current config
    Link {
        /// Path to a file or directory
        #[arg()]
        path: PathBuf,
    },
    /// Unlink a path from the current config
    Unlink {
        /// Path to a file or directory
        #[arg()]
        path: PathBuf,
    },
    /// Generate shell completions
    Completions,
}
