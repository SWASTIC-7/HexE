use super::lexer;

use lexer::Instruction;
#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    Directive(String),
    Instruction(Instruction),
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct ParsedToken {
    label: Option<String>,
    command: Command,
    operand: Option<String>,
}
#[warn(unused_mut)]
pub fn parser(buffer: &str) {
    let lexed_token = lexer::tokenize(buffer);
    for el in lexed_token.iter() {
        let mut dir: String = String::new();
        let mut instr = Instruction::default();
        let mut lab: String = String::new();
        let mut opr: String = String::new();
        for token in el.iter() {
            match token {
                lexer::Token::Directive(directive) => {
                    dir = directive.clone();
                }
                lexer::Token::Instruction(instruction) => {
                    instr = instruction.clone();
                }
                lexer::Token::Label(label) => {
                    lab = label.clone();
                }
                lexer::Token::Operand(operand) => {
                    opr = operand.clone();
                }
            }
        }
        let command = if !dir.is_empty() {
            Command::Directive(dir)
        } else {
            Command::Instruction(instr)
        };
        let parsed_token = ParsedToken {
            label: if lab.is_empty() { None } else { Some(lab) },
            command,
            operand: if opr.is_empty() { None } else { Some(opr) },
        };
        println!("{parsed_token:?}");
    }
}
