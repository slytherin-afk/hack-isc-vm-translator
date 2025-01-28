use regex::Regex;

use crate::commands::Command;

enum FunctionalCommandType {
    Call,
    Function,
    Return,
}

pub struct FunctionalCommand<'a> {
    file_name: &'a str,
    function_name: &'a str,
    calling_function: Option<&'a str>,
    ret: u16,
    arg: u16,
    command_type: FunctionalCommandType,
}

impl<'a> FunctionalCommand<'a> {
    pub fn new(
        command: &'a str,
        file_name: &'a str,
        function_name: Option<&'a str>,
        ret: u16,
    ) -> Option<Self> {
        let patterns = [
            (
                FunctionalCommandType::Function,
                r"function\s+([\w\.]+)\s+(\d+)",
            ),
            (FunctionalCommandType::Return, r"return"),
            (FunctionalCommandType::Call, r"call\s+([\w\.]+)\s+(\d+)"),
        ];

        for (_i, (command_type, pattern)) in patterns.into_iter().enumerate() {
            if let Ok(regex) = Regex::new(pattern) {
                if let Some(captures) = regex.captures(command) {
                    let arg = if let Some(label) = captures.get(2) {
                        label.as_str().parse().unwrap()
                    } else {
                        0
                    };

                    let calling_function = if let Some(label) = captures.get(1) {
                        Some(label.as_str())
                    } else {
                        None
                    };

                    return Some(Self {
                        arg,
                        calling_function,
                        command_type,
                        file_name,
                        function_name: function_name?,
                        ret,
                    });
                }
            }
        }

        None
    }
}

impl<'a> FunctionalCommand<'a> {
    fn generate_function_command(&self) -> Vec<String> {
        let mut result: Vec<String> = vec![format!(
            "({0})",
            self.calling_function.expect("to be a calling function")
        )];

        for _ in 0..self.arg {
            result.extend(vec![
                "@SP".to_string(),
                "M=M+1".to_string(),
                "A=M-1".to_string(),
                "M=0".to_string(),
            ]);
        }

        return result;
    }

    fn generate_return_command(&self) -> Vec<String> {
        vec![
            // save return address lcl - 5
            "@5".to_string(),
            "D=A".to_string(),
            "@LCL".to_string(),
            "A=M-D".to_string(),
            "D=M".to_string(),
            "@13".to_string(),
            "M=D".to_string(),
            // push return to arg 0
            "@SP".to_string(),
            "A=M-1".to_string(),
            "D=M".to_string(),
            "@ARG".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
            // put sp back arg + 1
            "@ARG".to_string(),
            "D=M+1".to_string(),
            "@SP".to_string(),
            "M=D".to_string(),
            // put that back lcl - 1
            "@1".to_string(),
            "D=A".to_string(),
            "@LCL".to_string(),
            "A=M-D".to_string(),
            "D=M".to_string(),
            "@THAT".to_string(),
            "M=D".to_string(),
            // put this back lcl - 2
            "@2".to_string(),
            "D=A".to_string(),
            "@LCL".to_string(),
            "A=M-D".to_string(),
            "D=M".to_string(),
            "@THIS".to_string(),
            "M=D".to_string(),
            // put arg back lcl - 3
            "@3".to_string(),
            "D=A".to_string(),
            "@LCL".to_string(),
            "A=M-D".to_string(),
            "D=M".to_string(),
            "@ARG".to_string(),
            "M=D".to_string(),
            // put lcl back lcl - 4
            "@4".to_string(),
            "D=A".to_string(),
            "@LCL".to_string(),
            "A=M-D".to_string(),
            "D=M".to_string(),
            "@LCL".to_string(),
            "M=D".to_string(),
            // goto
            "@13".to_string(),
            "A=M".to_string(),
            "0;JMP".to_string(),
        ]
    }

    fn generate_call_command(&self) -> Vec<String> {
        let return_label = format!(
            "{0}$ret.{1}",
            self.function_name, self.ret
        );
        let narg = 5 + self.arg;
        let calling_function = format!("@{0}", self.calling_function.expect("a calling function"));

        vec![
            // push return address
            format!("@{return_label}"),
            "D=A".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
            "A=M-1".to_string(),
            "M=D".to_string(),
            // push lcl
            "@LCL".to_string(),
            "D=M".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
            "A=M-1".to_string(),
            "M=D".to_string(),
            // push arg
            "@ARG".to_string(),
            "D=M".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
            "A=M-1".to_string(),
            "M=D".to_string(),
            // push this
            "@THIS".to_string(),
            "D=M".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
            "A=M-1".to_string(),
            "M=D".to_string(),
            // push that
            "@THAT".to_string(),
            "D=M".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
            "A=M-1".to_string(),
            "M=D".to_string(),
            // set arg
            format!("@{narg}"),
            "D=A".to_string(),
            "@SP".to_string(),
            "D=M-D".to_string(),
            "@ARG".to_string(),
            "M=D".to_string(),
            // set lcl = sp
            "@SP".to_string(),
            "D=M".to_string(),
            "@LCL".to_string(),
            "M=D".to_string(),
            // goto function call
            calling_function,
            "0;JMP".to_string(),
            // return label at the end
            format!("({return_label})"),
        ]
    }
}

impl<'a> Command for FunctionalCommand<'a> {
    fn generate(&self) -> Vec<String> {
        match self.command_type {
            FunctionalCommandType::Call => self.generate_call_command(),
            FunctionalCommandType::Function => self.generate_function_command(),
            FunctionalCommandType::Return => self.generate_return_command(),
        }
    }
}
