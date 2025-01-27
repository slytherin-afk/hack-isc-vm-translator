enum ArithmeticType {
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

use crate::commands::Command;
use ArithmeticType::*;

pub struct ArithmeticCommand<'a> {
    command_type: ArithmeticType,
    command: &'a str,
}

impl<'a> ArithmeticCommand<'a> {
    pub fn new(command: &'a str) -> Option<Self> {
        let command_type = match command {
            "add" => Add,
            "sub" => Sub,
            "neg" => Neg,
            "eq" => Eq,
            "gt" => Gt,
            "lt" => Lt,
            "and" => And,
            "or" => Or,
            "not" => Not,
            _ => return None,
        };

        return Some(Self {
            command_type,
            command,
        });
    }
}

impl<'a> ArithmeticCommand<'a> {
    fn generate_2_operand_arithmetic_code(&self) -> Vec<String> {
        let main_isc = match self.command_type {
            Add => "D=D+M",
            Sub => "D=D-M",
            And => "D=D&M",
            Or => "D=D|M",
            _ => unreachable!(),
        };

        vec![
            "@SP".to_string(),
            "AM=M-1".to_string(),
            "D=M".to_string(),
            "@R13".to_string(),
            "M=D".to_string(),
            "@SP".to_string(),
            "A=M-1".to_string(),
            "D=M".to_string(),
            "@R13".to_string(),
            main_isc.to_string(),
            "@SP".to_string(),
            "A=M-1".to_string(),
            "M=D".to_string(),
        ]
    }

    fn generate_not_operand_arithmetic_code(&self) -> Vec<String> {
        vec!["@SP".to_string(), "A=M-1".to_string(), "M=!M".to_string()]
    }

    fn generate_neg_operand_arithmetic_code(&self) -> Vec<String> {
        vec!["@SP".to_string(), "A=M-1".to_string(), "M=-M".to_string()]
    }

    fn generate_comparison_arithmetic_code(&self) -> Vec<String> {
        let main_isc = match self.command_type {
            Eq => "D;JEQ",
            Gt => "D;JGT",
            Lt => "D;JLT",
            _ => unreachable!(),
        };

        vec![
            "@SP".to_string(),
            "AM=M-1".to_string(),
            "D=M".to_string(),
            "@SP".to_string(),
            "A=M-1".to_string(),
            "A=M".to_string(),
            "D=A-D".to_string(),
            "@MINUS_ONE".to_string(),
            main_isc.to_string(),
            "@SP".to_string(),
            "A=M-1".to_string(),
            "M=0".to_string(),
            "@END".to_string(),
            "0;JMP".to_string(),
            "(MINUS_ONE)".to_string(),
            "@SP".to_string(),
            "A=M-1".to_string(),
            "M=-1".to_string(),
            "(END)".to_string(),
        ]
    }
}

impl<'a> Command for ArithmeticCommand<'a> {
    fn generate(&self) -> Vec<String> {
        match self.command_type {
            Add | Sub | And | Or => self.generate_2_operand_arithmetic_code(),
            Neg => self.generate_neg_operand_arithmetic_code(),
            Not => self.generate_not_operand_arithmetic_code(),
            Eq | Gt | Lt => self.generate_comparison_arithmetic_code(),
        }
    }
}
