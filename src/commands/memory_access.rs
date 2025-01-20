use regex::Regex;
enum MemorySegment {
    Lcl,
    Arg,
    This,
    That,
    Pointer,
    Temp,
    Static,
    Constant,
}

enum MemoryCommandType {
    Push,
    Pop,
}

use MemoryCommandType::*;
use MemorySegment::*;

use crate::commands::Command;

pub struct MemoryAccessCommand<'a> {
    file_name: &'a str,
    command_type: MemoryCommandType,
    segment: MemorySegment,
    command: &'a str,
    i: i16,
}

impl<'a> MemoryAccessCommand<'a> {
    pub fn new(command: &'a str, file_name: &'a str) -> Option<Self> {
        let memory_access_command_pattern = Regex::new(
            r"^(push|pop)\s+(argument|local|this|that|static|pointer|temp|constant)\s+(\d+)$",
        )
        .ok()?;

        let captures = memory_access_command_pattern.captures(command)?;

        let command_type = match &captures[2] {
            "pop" => Pop,
            "push" => Push,
            _ => unreachable!(),
        };

        let segment = match &captures[2] {
            "argument" => Arg,
            "local" => Lcl,
            "this" => This,
            "that" => That,
            "static" => Static,
            "pointer" => Pointer,
            "temp" => Temp,
            _ => unreachable!(),
        };

        let i = (&captures[3]).parse::<i16>().ok()?;

        if let MemorySegment::Pointer = segment {
            if i != 0 && i != 1 {
                return None;
            }
        }

        Some(Self {
            file_name,
            command_type,
            segment,
            command,
            i,
        })
    }
}

impl<'a> MemoryAccessCommand<'a> {
    fn handle_segment_push(&self) -> Vec<String> {
        let segment = match self.segment {
            Lcl => "LCL",
            Arg => "ARG",
            This => "THIS",
            That => "THAT",
            _ => unreachable!(),
        };

        vec![
            format!("@{0}", self.i),
            "D=A".to_string(),
            format!("@{segment}"),
            "A=D+M".to_string(),
            "D=M".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
            "A=M-1".to_string(),
            "M=D".to_string(),
        ]
    }

    fn handle_constant_push(&self) -> Vec<String> {
        vec![
            format!("@{0}", self.i),
            "D=A".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
            "A=M-1".to_string(),
            "M=D".to_string(),
        ]
    }

    fn handle_pointer_push(&self) -> Vec<String> {
        let segment = match self.i {
            0 => "THIS",
            1 => "THAT",
            _ => unreachable!(),
        };

        vec![
            format!("@{segment}"),
            "D=M".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
            "A=M-1".to_string(),
            "M=D".to_string(),
        ]
    }

    fn handle_static_push(&self) -> Vec<String> {
        vec![
            format!("@{0}.{1}", self.file_name, self.i),
            "D=M".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
            "A=M-1".to_string(),
            "M=D".to_string(),
        ]
    }

    fn handle_temp_push(&self) -> Vec<String> {
        vec![
            format!("@{0}", &self.i + 5),
            "D=M".to_string(),
            "@SP".to_string(),
            "M=M+1".to_string(),
            "A=M-1".to_string(),
            "M=D".to_string(),
        ]
    }
}

impl<'a> MemoryAccessCommand<'a> {
    fn handle_segment_pop(&self) -> Vec<String> {
        let segment = match self.segment {
            Lcl => "LCL",
            Arg => "ARG",
            This => "THIS",
            That => "THAT",
            _ => unreachable!(),
        };

        vec![
            format!("@{0}", self.i),
            "D=A".to_string(),
            format!("@{segment}"),
            "D=D+M".to_string(),
            "@R13".to_string(),
            "M=D".to_string(),
            "@SP".to_string(),
            "AM=M-1".to_string(),
            "D=M".to_string(),
            "@R13".to_string(),
            "A=M".to_string(),
            "M=D".to_string(),
        ]
    }

    fn handle_constant_pop(&self) -> Vec<String> {
        vec!["@SP".to_string(), "AM=M-1".to_string()]
    }

    fn handle_pointer_pop(&self) -> Vec<String> {
        let segment = match self.i {
            0 => "THIS",
            1 => "THAT",
            _ => unreachable!(),
        };

        vec![
            "@SP".to_string(),
            "AM=M-1".to_string(),
            "D=M".to_string(),
            format!("@{segment}"),
            "M=D".to_string(),
        ]
    }

    fn handle_static_pop(&self) -> Vec<String> {
        vec![
            "@SP".to_string(),
            "AM=M-1".to_string(),
            "D=M".to_string(),
            format!("@{0}.{1}", self.file_name, self.i),
            "M=D".to_string(),
        ]
    }

    fn handle_temp_pop(&self) -> Vec<String> {
        vec![
            "@SP".to_string(),
            "AM=M-1".to_string(),
            "D=M".to_string(),
            format!("@{0}", &self.i + 5),
            "M=D".to_string(),
        ]
    }
}

impl<'a> Command for MemoryAccessCommand<'a> {
    fn generate(&self) -> Vec<String> {
        match self.command_type {
            Push => match self.segment {
                Lcl | Arg | This | That => self.handle_segment_push(),
                Pointer => self.handle_pointer_push(),
                Temp => self.handle_temp_push(),
                Static => self.handle_static_push(),
                Constant => self.handle_constant_push(),
            },
            Pop => match self.segment {
                Lcl | Arg | This | That => self.handle_segment_pop(),
                Pointer => self.handle_pointer_pop(),
                Temp => self.handle_temp_pop(),
                Static => self.handle_static_pop(),
                Constant => self.handle_constant_pop(),
            },
        }
    }
}
