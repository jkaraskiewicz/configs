use std::path::Path;

use colored::Colorize;

pub trait Colorized {
    fn colorize(&self, as_source: bool) -> String;
}

impl Colorized for Path {
    fn colorize(&self, as_source: bool) -> String {
        let file_name = self.file_name().unwrap().to_str().unwrap();
        let prefix = self.to_str().unwrap().strip_suffix(file_name).unwrap();

        let file_name = if as_source {
            file_name.magenta()
        } else if self.is_dir() {
            file_name.blue()
        } else {
            file_name.white()
        };

        let prefix = if as_source {
            prefix.magenta()
        } else {
            prefix.cyan()
        };

        format!("{}{}", prefix, file_name)
    }
}
