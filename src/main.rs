use escpos::job::instruction::{CutInstruction, TextInstruction};
use escpos::job::PrintJob;

fn main() -> Result<(), String> {
    let mut printer = escpos::Model::TmT88v.get();

    printer.set_default_line_spacing()?;

    printer.set_line_spacing(50)?;

    PrintJob::default()
        .add_instruction(TextInstruction::new_line("Line 0".to_string()))
        .add_instruction(TextInstruction::new_line("Line 1".to_string()))
        .add_instruction(CutInstruction::new())
        .print(&mut printer)?;

    Ok(())
}
