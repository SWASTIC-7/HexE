pub enum token {
    label(String),
    instruction(String),
    operand(String),
    comment(String),
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
        labeling(&el);
        println!("{:?}", el);
    }
}

fn labeling(token_line: &Vec<String>){
    
}