use anyhow::Result;
use std::{fs, path::Path};

use crate::commands::{Command, CommandType};

pub struct Parser {
    pub file: String,
    pub file_name: String,
    pub nth: usize,
    pub function_name: Option<String>,
    pub ret: u16,
}

impl Parser {
    pub fn new(file_path: &Path) -> Result<Self> {
        let file = fs::read_to_string(file_path)?;

        Ok(Self {
            file,
            file_name: file_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            function_name: None,
            ret: 0,
            nth: 0,
        })
    }

    fn clean(command: &str) -> &str {
        command.split("//").nth(0).unwrap().trim()
    }

    pub fn has_more_command(&mut self) -> bool {
        let mut iterator = self.file.lines().skip(self.nth).enumerate();
        let mut counter = self.nth;

        while let Some((n, c)) = iterator.next() {
            let c = c.trim();
            counter = n + 1;

            if c.starts_with("//") || c.len() <= 0 {
                continue;
            }

            self.nth = self.nth + counter;

            return true;
        }

        return false;
    }

    pub fn advance<'a>(&'a self) -> Box<dyn Command + 'a> {
        let command: &str = Self::clean(
            self.file
                .lines()
                .nth(self.nth - 1)
                .expect(&format!("a command {0}", self.nth)),
        );

        return CommandType::new(
            command,
            &self.file_name,
            match &self.function_name {
                Some(name) => Some(name),
                None => None,
            },
        )
        .expect(&format!("valid command type {}", command));
    }
}
