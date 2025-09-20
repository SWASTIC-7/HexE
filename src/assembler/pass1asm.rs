use super::parser::Command;
use super::parser::parser;

pub struct SymbolTable {}

pub fn pass1asm(buffer: &str) {
    let parsed_lines = parser(buffer);
    let mut Locctr = 0;
    for lines in parsed_lines.iter() {
        match &lines.command {
            Command::Instruction(instr) => {
                if instr.instr == "START" {
                    let operand: Option<String> = lines.operand1.clone();
                    let num: Option<i32> = operand.as_ref().and_then(|s| s.parse::<i32>().ok());
                    if let Some(value) = num {
                        Locctr = value;
                    } else {
                        println!("Error getting the valid startig address");
                    }
                }
            }
            _ => {}
        }
    }
}
