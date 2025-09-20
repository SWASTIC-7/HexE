use super::lexer;

use lexer::Instruction;
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Command {
    Directive(String),
    Instruction(Instruction),
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ParsedToken {
    label: Option<String>,
    pub command: Command,
    pub operand1: Option<String>,
    operand2: Option<String>,
}
#[warn(unused_mut)]
pub fn parser(buffer: &str) -> Vec<ParsedToken> {
    let lexed_token = lexer::tokenize(buffer);
    let mut parsed_lines: Vec<ParsedToken> = Vec::new();
    for el in lexed_token.iter() {
        let mut dir: String = String::new();
        let mut instr = Instruction::default();
        let mut lab: String = String::new();
        let mut opr1: String = String::new();
        let mut opr2: String = String::new();
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
                lexer::Token::Operand1(operand1) => {
                    opr1 = operand1.clone();
                }
                lexer::Token::Operand2(operand2) => {
                    opr2 = operand2.clone();
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
            operand1: if opr1.is_empty() { None } else { Some(opr1) },
            operand2: if opr2.is_empty() { None } else { Some(opr2) },
        };
        parsed_lines.push(parsed_token);
        // println!("{parsed_token:?}");
    }
    parsed_lines
}
