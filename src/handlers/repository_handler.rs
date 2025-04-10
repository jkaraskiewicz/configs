use std::env;

use anyhow::Result;
use versions::{open, Repository};

pub fn get_current_repository() -> Result<Repository> {
    let current_dir = env::current_dir()?;
    let repository = open(&current_dir, true)?;
    Ok(repository)
}
