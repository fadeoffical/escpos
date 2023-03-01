pub mod instruction;

use crate::job::instruction::Instruction;
use crate::Printer;

pub struct PrintJob {
    instructions: Vec<Box<dyn Instruction>>,
}

impl Default for PrintJob {
    fn default() -> Self {
        Self::new()
    }
}

impl PrintJob {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(mut self, instruction: Box<dyn Instruction>) -> Self {
        self.instructions.push(instruction);
        self
    }

    pub fn print(self, printer: &mut Box<dyn Printer>) -> Result<(), String> {
        printer.init().unwrap();

        for instruction in self.instructions {
            instruction.write(printer).unwrap();
        }

        printer.finish().unwrap();
        Ok(())
    }
}
