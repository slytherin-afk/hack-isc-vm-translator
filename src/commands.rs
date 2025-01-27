pub mod arithmetic;
pub mod branching;
pub mod memory_access;

use arithmetic::ArithmeticCommand;
use branching::BranchingCommand;
use memory_access::MemoryAccessCommand;

pub trait Command {
    fn generate(&self) -> Vec<String>;
}

pub struct CommandType;

impl CommandType {
    pub fn new<'a>(
        command: &'a str,
        file_name: &'a str,
        function_name: Option<&'a str>,
    ) -> Option<Box<dyn Command + 'a>> {
        let assume_arithmetic: Option<ArithmeticCommand<'_>> = ArithmeticCommand::new(command);

        if let Some(command) = assume_arithmetic {
            return Some(Box::new(command));
        }

        let assume_memory_access = MemoryAccessCommand::new(command, file_name);

        if let Some(command) = assume_memory_access {
            return Some(Box::new(command));
        }

        let assume_branching = BranchingCommand::new(command, file_name, function_name);

        if let Some(command) = assume_branching {
            return Some(Box::new(command));
        }

        None
    }
}
