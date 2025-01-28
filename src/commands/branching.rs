use regex::Regex;

use crate::commands::Command;

enum BranchingCommandType {
    IfGoto,
    Goto,
    Label,
}

pub struct BranchingCommand<'a> {
    label: String,
    file_name: &'a str,
    function_name: Option<&'a str>,
    command_type: BranchingCommandType,
}

impl<'a> BranchingCommand<'a> {
    pub fn new(
        command: &'a str,
        file_name: &'a str,
        function_name: Option<&'a str>,
    ) -> Option<Self> {
        let patterns = [
            (BranchingCommandType::IfGoto, r"if-goto\s+(\w+)"),
            (BranchingCommandType::Goto, r"goto\s+(\w+)"),
            (BranchingCommandType::Label, r"label\s+(\w+)"),
        ];

        for (command_type, pattern) in patterns.into_iter() {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(captures) = regex.captures(command) {
                    if let Some(label) = captures.get(1) {
                        return Some(Self {
                            label: label.as_str().to_string(),
                            file_name,
                            function_name,
                            command_type,
                        });
                    }
                }
            }
        }

        None
    }
}

impl<'a> BranchingCommand<'a> {
    fn generate_if_goto(&self) -> Vec<String> {
        vec![
            "@SP".to_string(),
            "AM=M-1".to_string(),
            "D=M".to_string(),
            format!("@{0}${1}", self.function_name.unwrap_or(""), self.label),
            "D;JNE".to_string(),
        ]
    }

    fn generate_goto(&self) -> Vec<String> {
        vec![
            format!("@{0}${1}", self.function_name.unwrap_or(""), self.label),
            "0;JMP".to_string(),
        ]
    }

    fn generate_label(&self) -> Vec<String> {
        vec![format!(
            "({0}${1})",
            self.function_name.unwrap_or(""),
            self.label
        )]
    }
}

impl<'a> Command for BranchingCommand<'a> {
    fn generate(&self) -> Vec<String> {
        match self.command_type {
            BranchingCommandType::IfGoto => self.generate_if_goto(),
            BranchingCommandType::Goto => self.generate_goto(),
            BranchingCommandType::Label => self.generate_label(),
        }
    }
}
