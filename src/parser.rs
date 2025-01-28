use anyhow::Result;
use regex::Regex;
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
        let mut counter;

        while let Some((n, c)) = iterator.next() {
            let c = c.trim();
            counter = n + 1;

            if c.starts_with("//") || c.len() <= 0 {
                continue;
            }

            let function_pattern =
                Regex::new(r"function\s+([\w\.]+)\s+(\d+)").expect("to be regex");

            if let Some(pat) = function_pattern.captures(c) {
                self.function_name =
                    Some(pat.get(1).expect("a function name").as_str().to_string());
                self.ret = 0;
            }

            let call_pattern = Regex::new(r"call\s+([\w\.]+)\s+(\d+)").expect("to be regex");

            if let Some(_) = call_pattern.captures(c) {
                self.ret += 1;
            }

            self.nth = self.nth + counter;

            return true;
        }

        return false;
    }

    pub fn advance<'a>(&'a self) -> (String, Box<dyn Command + 'a>) {
        let command: &str = Self::clean(
            self.file
                .lines()
                .nth(self.nth - 1)
                .expect(&format!("a command [{0}]", self.nth)),
        );

        return (
            command.to_string(),
            CommandType::new(
                command,
                &self.file_name,
                match &self.function_name {
                    Some(name) => Some(name),
                    None => None,
                },
                self.ret,
            )
            .expect(&format!("valid command type [{}]", command)),
        );
    }
}
