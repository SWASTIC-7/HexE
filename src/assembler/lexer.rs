use super::directive;
use super::opcode;
use super::opcode::OpCode;
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Token {
    Label(String),
    Instruction(Instruction),
    Directive(String),
    Operand1(String),
    Operand2(String),
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct Instruction {
    pub instr: String,
    opcode: OpCode,
}

#[warn(unused_assignments)]
pub fn tokenize(buffer: &str) -> Vec<Vec<Token>> {
    let mut lexed_token: Vec<Vec<Token>> = Vec::new();
    let token_line = segregate(buffer);
    for el in token_line.iter() {
        if el.is_empty() {
            continue;
        }
        let labeled_token: Vec<Token> = labeling(el);
        lexed_token.push(labeled_token);
        // println!("{:?}", labeled_token);
    }
    lexed_token
}

fn segregate(buffer: &str) -> Vec<Vec<String>> {
    let mut token_vec: Vec<String> = Vec::new();
    let mut token_line: Vec<Vec<String>> = Vec::new();
    for l in buffer.lines() {
        let mut new_token: String = String::new();
        for c in l.chars() {
            if c == ' ' && !new_token.is_empty() {
                token_vec.push(new_token.clone());
                new_token.clear();
            }
            if c == '.' {
                //breaking the loop of line before comment
                break;
            }
            if c != ' ' {
                new_token.push(c);
            }
            if c == ',' {
                token_vec.push(new_token.clone());
                new_token.clear();
                continue;
            }
        }
        if new_token.is_empty() {
            continue;
        }
        token_vec.push(new_token.clone());
        new_token.clear();
        token_line.push(token_vec.clone());
        token_vec.clear();
    }
    token_line
}

//giving label enum(token) to the fetched tokens
fn labeling(token_line: &[String]) -> Vec<Token> {
    let opcode = opcode::build_optab();
    let mut arr: Vec<Token> = Vec::new();
    let mut check: bool = false;
    let mut check_for_op1: bool = false;
    for tokens in token_line.iter() {
        if !check {
            if let Some(code) = opcode.get(tokens.as_str()) {
                let instr = Instruction {
                    instr: tokens.clone(),
                    opcode: code.clone(),
                };
                arr.push(Token::Instruction(instr));
                check = true;
            } else if directive::directives().contains(tokens) {
                arr.push(Token::Directive(tokens.clone()));
                check = true;
            } else {
                arr.push(Token::Label(tokens.clone()));
            }
        } else {
            if !check_for_op1 {
                arr.push(Token::Operand1(tokens.clone()));
                check_for_op1 = true;
            } else {
                arr.push(Token::Operand2(tokens.clone()));
            }
        }
    }
    arr
}
