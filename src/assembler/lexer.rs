use super::directive;
use super::opcode;
use super::opcode::OpCode;
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Token {
    Label(String),
    Instruction(Instruction),
    Directive(String),
    Operand(String),
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Instruction {
    instr: String,
    opcode: OpCode,
}

pub fn tokenize(buffer: &str) {
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
        }
        if new_token == "" {
            continue;
        }
        token_vec.push(new_token.clone());
        new_token.clear();
        token_line.push(token_vec.clone());
        token_vec.clear();
    }
    for el in token_line.iter() {
        if el.is_empty() {
            continue;
        }
        let labeled_token: Vec<Token> = labeling(el);
        println!("{:?}", labeled_token);
    }
}

//giving label enum(token) to the fetched tokens
fn labeling(token_line: &Vec<String>) -> Vec<Token> {
    let opcode = opcode::build_optab();
    let mut arr: Vec<Token> = Vec::new();
    let mut check: bool = false;
    for tokens in token_line.iter() {
        if !check {
            if let Some(code) = opcode.get(tokens.as_str()) {
                let instr = Instruction {
                    instr: tokens.clone(),
                    opcode: code.clone(),
                };
                arr.push(Token::Instruction(instr));
                check = true;
            } else if directive::directives().contains(&tokens) {
                arr.push(Token::Directive(tokens.clone()));
                check = true;
            } else {
                arr.push(Token::Label(tokens.clone()));
            }
        } else {
            arr.push(Token::Operand(tokens.clone()));
        }
    }
    arr
}
