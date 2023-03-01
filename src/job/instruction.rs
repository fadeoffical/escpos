use crate::Printer;

pub trait Instruction {
    fn write(&self, printer: &mut Box<dyn Printer>) -> Result<(), String>;
}

pub struct CutInstruction {
    feed: u8,
}

impl CutInstruction {
    pub fn new() -> Box<Self> {
        let instruction = Self {
            feed: 5,
        };
        Box::new(instruction)
    }
}

impl Instruction for CutInstruction {
    fn write(&self, printer: &mut Box<dyn Printer>) -> Result<(), String> {
        printer.feed(self.feed)?;
        printer.cut()?;
        Ok(())
    }
}

pub struct TextInstruction {
    text: String,
}

impl TextInstruction {
    pub fn new(text: String) -> Box<Self> {
        let instruction = Self {
            text,
        };
        Box::new(instruction)
    }

    pub fn new_line(text: String) -> Box<Self> {
        let instruction = Self {
            text: format!("{}\n", text),
        };
        Box::new(instruction)
    }
}

impl Instruction for TextInstruction {
    fn write(&self, printer: &mut Box<dyn Printer>) -> Result<(), String> {
        printer.writeln(self.text.as_bytes())?;
        printer.flush()?;
        Ok(())
    }
}
