pub mod arithmetic;
pub mod memory_access;

use arithmetic::ArithmeticCommand;
use memory_access::MemoryAccessCommand;

pub trait Command {
    fn generate(&self) -> Vec<String>;
}

pub struct CommandType;

impl CommandType {
    pub fn new<'a>(command: &'a str, file_name: &'a str) -> Option<Box<dyn Command + 'a>> {
        let assume_arithmetic = ArithmeticCommand::new(command);

        if let Some(command) = assume_arithmetic {
            return Some(Box::new(command));
        }

        let assume_memory_access = MemoryAccessCommand::new(command, file_name);

        if let Some(command) = assume_memory_access {
            return Some(Box::new(command));
        }

        None
    }
}
