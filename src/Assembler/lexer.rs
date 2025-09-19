use super::opcode;
use super::opcode::OpCode;
pub enum token_label {
    label(String),
    instruction(instruction),
    operand(String),
    comment(String),
}

pub struct instruction {
    instr: String,
    opcode: OpCode,
}

pub struct token {
    token_label: token_label,
    token_val: String,
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
        token_vec.push(new_token.clone());
        new_token.clear();
        token_line.push(token_vec.clone());
        token_vec.clear();
    }

    for el in token_line.iter() {
        labeling(el);
        println!("{:?}", el);
    }
}

//giving label enum(token) to the fetched tokens
fn labeling(token_line: &Vec<String>) -> Vec<token> {
    let opcode = opcode::build_optab();
    let arr: Vec<token> = Vec::new();
    for tokens in token_line.iter() {
        if let Some(code) = opcode.get(tokens.as_str()) {
            print!("opcode found")
        } else {
            println!("opcode not found");
        }
    }
    arr
}
