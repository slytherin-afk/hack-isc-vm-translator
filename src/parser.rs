use anyhow::Result;
use std::{fs, path::Path};

use crate::commands::{Command, CommandType};

pub struct Parser {
    pub file: String,
    pub file_name: String,
    nth: usize,
}

impl Parser {
    pub fn construct(file_path: &Path) -> Result<Self> {
        let file = fs::read_to_string(file_path)?;

        Ok(Self {
            file,
            file_name: file_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            nth: 0,
        })
    }

    fn clean(command: &str) -> &str {
        command.split("//").nth(0).unwrap().trim()
    }

    pub fn has_more_command(&mut self) -> bool {
        let mut iterator = self.file.lines().skip(self.nth).enumerate();

        while let Some((n, c)) = iterator.next() {
            let c = c.trim();
            self.nth = n;

            if c.starts_with("//") || c.len() <= 0 {
                continue;
            }

            return true;
        }

        return false;
    }

    pub fn advance<'a>(&'a self) -> Box<dyn Command + 'a> {
        let command = Self::clean(self.file.lines().nth(self.nth).expect("a command"));

        return CommandType::new(command, &self.file_name).expect("valid command type");
    }
}
