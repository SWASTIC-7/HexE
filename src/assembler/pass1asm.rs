use super::parser::parser;
use crate::error::{log_error, log_info};
use crate::predefined::common::{
    Command, LITERALTABLE, LabeledParsedLines, LiteralTable, SYMBOLTABLE, SymbolTable,
};

fn parse_literal(literal: &str) -> Option<(String, u32)> {
    if let Some(stripped) = literal.strip_prefix('=') {
        if let Some(char_lit) = stripped.strip_prefix("C'") {
            if let Some(content) = char_lit.strip_suffix('\'') {
                let hex_value: String = content.bytes().map(|b| format!("{:02X}", b)).collect();
                let length = content.len() as u32;
                return Some((hex_value, length));
            }
        } else if let Some(hex_lit) = stripped.strip_prefix("X'")
            && let Some(content) = hex_lit.strip_suffix('\'')
        {
            let length = (content.len() / 2) as u32;
            return Some((content.to_uppercase(), length));
        }
    }
    None
}

fn is_literal(operand: &str) -> bool {
    operand.starts_with('=')
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Number(i32),
    Symbol(usize),
    Add,
    Sub,
    Mul,
    Div,
    LParen,
    RParen,
}

fn tokenize_expression(expr: &str, symbol_table: &[SymbolTable]) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();

    for ch in expr.chars() {
        match ch {
            '+' => {
                if !current.is_empty() {
                    tokens.push(parse_operand(&current, symbol_table)?);
                    current.clear();
                }
                tokens.push(Token::Add);
            }
            '-' => {
                if !current.is_empty() {
                    tokens.push(parse_operand(&current, symbol_table)?);
                    current.clear();
                }
                tokens.push(Token::Sub);
            }
            '*' => {
                if !current.is_empty() {
                    tokens.push(parse_operand(&current, symbol_table)?);
                    current.clear();
                }
                tokens.push(Token::Mul);
            }
            '/' => {
                if !current.is_empty() {
                    tokens.push(parse_operand(&current, symbol_table)?);
                    current.clear();
                }
                tokens.push(Token::Div);
            }
            '(' => {
                if !current.is_empty() {
                    tokens.push(parse_operand(&current, symbol_table)?);
                    current.clear();
                }
                tokens.push(Token::LParen);
            }
            ')' => {
                if !current.is_empty() {
                    tokens.push(parse_operand(&current, symbol_table)?);
                    current.clear();
                }
                tokens.push(Token::RParen);
            }
            ' ' | '\t' => {}
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.is_empty() {
        tokens.push(parse_operand(&current, symbol_table)?);
    }

    Ok(tokens)
}

fn parse_operand(operand: &str, symbol_table: &[SymbolTable]) -> Result<Token, String> {
    // Try to parse as decimal number
    if let Ok(num) = operand.parse::<i32>() {
        return Ok(Token::Number(num));
    }

    // Try to parse as hexadecimal (0x prefix or $ prefix)
    if let Some(hex_str) = operand
        .strip_prefix("0x")
        .or_else(|| operand.strip_prefix("0X"))
        && let Ok(num) = i32::from_str_radix(hex_str, 16)
    {
        return Ok(Token::Number(num));
    }

    if let Some(hex_str) = operand.strip_prefix('$')
        && let Ok(num) = i32::from_str_radix(hex_str, 16)
    {
        return Ok(Token::Number(num));
    }

    if let Some(index) = symbol_table.iter().position(|sym| sym.label == operand) {
        return Ok(Token::Symbol(index));
    }

    Err(format!("Unknown symbol or invalid number: {}", operand))
}

// Evaluate expression using operator precedence (Shunting Yard algorithm)
fn evaluate_expression(tokens: Vec<Token>, symbol_table: &[SymbolTable]) -> Result<i32, String> {
    let mut output: Vec<i32> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();

    fn precedence(op: &Token) -> i32 {
        match op {
            Token::Add | Token::Sub => 1,
            Token::Mul | Token::Div => 2,
            _ => 0,
        }
    }

    fn apply_operator(op: Token, output: &mut Vec<i32>) -> Result<(), String> {
        if output.len() < 2 {
            return Err("Invalid expression: insufficient operands".to_string());
        }

        let b = output.pop().unwrap();
        let a = output.pop().unwrap();

        let result = match op {
            Token::Add => a + b,
            Token::Sub => a - b,
            Token::Mul => a * b,
            Token::Div => {
                if b == 0 {
                    return Err("Division by zero".to_string());
                }
                a / b
            }
            _ => return Err("Invalid operator".to_string()),
        };

        output.push(result);
        Ok(())
    }

    for token in tokens {
        match token {
            Token::Number(n) => {
                output.push(n);
            }
            Token::Symbol(idx) => {
                let value = symbol_table[idx].address as i32;
                output.push(value);
            }
            Token::LParen => {
                operators.push(Token::LParen);
            }
            Token::RParen => {
                while let Some(op) = operators.last() {
                    if *op == Token::LParen {
                        break;
                    }
                    let op = operators.pop().unwrap();
                    apply_operator(op, &mut output)?;
                }

                if operators.is_empty() || operators.pop() != Some(Token::LParen) {
                    return Err("Mismatched parentheses".to_string());
                }
            }
            Token::Add | Token::Sub | Token::Mul | Token::Div => {
                while let Some(top) = operators.last() {
                    if *top == Token::LParen || precedence(top) < precedence(&token) {
                        break;
                    }
                    let op = operators.pop().unwrap();
                    apply_operator(op, &mut output)?;
                }
                operators.push(token);
            }
        }
    }

    while let Some(op) = operators.pop() {
        if op == Token::LParen || op == Token::RParen {
            return Err("Mismatched parentheses".to_string());
        }
        apply_operator(op, &mut output)?;
    }

    if output.len() != 1 {
        return Err("Invalid expression".to_string());
    }

    Ok(output[0])
}

pub fn expression_evaluate(expr: &str, symbol_table: &[SymbolTable]) -> Result<u32, String> {
    let tokens = tokenize_expression(expr, symbol_table)?;
    let result = evaluate_expression(tokens, symbol_table)?;

    if result < 0 {
        log_error(&format!(
            "Expression '{}' evaluated to negative value: {}",
            expr, result
        ));
        return Err(format!("Negative result: {}", result));
    }

    log_info(&format!("Expression '{}' = {}", expr, result));
    Ok(result as u32)
}

pub fn pass1asm(buffer: &str) -> (Vec<LabeledParsedLines>, u32, u32, Vec<SymbolTable>) {
    let parsed_lines = parser(buffer);
    let mut symbol_table = SYMBOLTABLE.lock().unwrap();
    let mut literal_table = LITERALTABLE.lock().unwrap();
    let mut labeledparsedline: Vec<LabeledParsedLines> = Vec::new();
    let mut locctr: u32 = 0x9999999;
    let mut length = 0;
    let mut startaddr = 0x00;
    let mut pending_literals: Vec<String> = Vec::new();

    for lines in parsed_lines.iter() {
        labeledparsedline.push(LabeledParsedLines {
            parsedtoken: lines.clone(),
            locctr,
        });

        match &lines.command {
            Command::Instruction(instr) => {
                if locctr != 0x9999999 {
                    // Check if operand is a literal
                    if let Some(operand) = &lines.operand1
                        && is_literal(operand)
                    {
                        // Add to literal table if not already present
                        if !literal_table.iter().any(|lit| lit.literal == *operand)
                            && !pending_literals.contains(operand)
                        {
                            pending_literals.push(operand.clone());

                            if let Some((value, lit_length)) = parse_literal(operand) {
                                literal_table.push(LiteralTable {
                                    literal: operand.clone(),
                                    value,
                                    length: lit_length,
                                    address: None,
                                });
                                log_info(&format!(
                                    "Found literal: {} (length: {} bytes)",
                                    operand, lit_length
                                ));
                            }
                        }
                    }

                    let format = instr.opcode.format;
                    if let Some(label) = lines.label.clone() {
                        symbol_table.push(SymbolTable {
                            label,
                            address: locctr,
                        });
                    }

                    locctr += format as u32;
                }
            }
            Command::Directive(directive) => {
                if locctr == 0x9999999 {
                    match directive.to_uppercase().as_str() {
                        "START" => {
                            let operand: Option<String> = lines.operand1.clone();
                            let num: Option<u32> =
                                operand.as_ref().and_then(|s| s.parse::<u32>().ok());
                            if let Some(value) = num {
                                startaddr = value;
                                locctr = value;
                                if let Some(label) = lines.label.clone() {
                                    symbol_table.push(SymbolTable {
                                        label,
                                        address: locctr,
                                    });
                                }
                            } else {
                                locctr = 0x00;
                            }
                        }
                        _ => {
                            println!("Did not get START");
                        }
                    }
                } else {
                    match directive.to_uppercase().as_str() {
                        "LTORG" | "END" => {
                            if !pending_literals.is_empty() {
                                log_info(&format!(
                                    "Allocating {} literals at {:06X}",
                                    pending_literals.len(),
                                    locctr
                                ));

                                for literal in &pending_literals {
                                    if let Some(lit_entry) = literal_table.iter_mut().find(|lit| {
                                        lit.literal == *literal && lit.address.is_none()
                                    }) {
                                        lit_entry.address = Some(locctr);
                                        log_info(&format!(
                                            "  Literal {} assigned address {:06X}",
                                            literal, locctr
                                        ));
                                        locctr += lit_entry.length;
                                    }
                                }
                                pending_literals.clear();
                            }

                            if directive.to_uppercase() == "END" {
                                length = locctr - startaddr;
                                break;
                            }
                        }
                        "EQU" => {
                            if let Some(label) = lines.label.clone() {
                                let operand: Option<String> = lines.operand1.clone();

                                let address: u32 = if let Some(expr) = &operand {
                                    // Try to evaluate as expression
                                    match expression_evaluate(expr, &symbol_table) {
                                        Ok(val) => val,
                                        Err(e) => {
                                            log_error(&format!(
                                                "Failed to evaluate EQU expression '{}': {}",
                                                expr, e
                                            ));
                                            locctr
                                        }
                                    }
                                } else {
                                    locctr
                                };

                                symbol_table.push(SymbolTable { label, address });
                            }
                        }
                        "WORD" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push(SymbolTable {
                                    label,
                                    address: locctr,
                                });
                            }
                            locctr += 3;
                        }
                        "RESW" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push(SymbolTable {
                                    label,
                                    address: locctr,
                                });
                            }
                            let operand: Option<String> = lines.operand1.clone();
                            let num: Option<u32> =
                                operand.as_ref().and_then(|s| s.parse::<u32>().ok());
                            if let Some(value) = num {
                                locctr += 3 * value;
                            }
                        }
                        "RESB" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push(SymbolTable {
                                    label,
                                    address: locctr,
                                });
                            }
                            let operand: Option<String> = lines.operand1.clone();
                            let num: Option<u32> =
                                operand.as_ref().and_then(|s| s.parse::<u32>().ok());
                            if let Some(value) = num {
                                locctr += value;
                            }
                        }
                        "BYTE" => {
                            if let Some(label) = lines.label.clone() {
                                symbol_table.push(SymbolTable {
                                    label,
                                    address: locctr,
                                });
                            }
                            let operand: Option<String> = lines.operand1.clone();
                            let len: Option<u32> = operand.as_ref().map(|s| s.len() as u32);
                            if let Some(value) = len {
                                locctr += value - 3;
                            }
                        }
                        _ => {
                            log_info(&format!("Unknown directive: {}", directive));
                        }
                    }
                }
            }
        }
    }

    // Log literal table summary
    log_info(&format!(
        "=== LITERAL TABLE ({} entries) ===",
        literal_table.len()
    ));
    for lit in literal_table.iter() {
        log_info(&format!(
            "  {} = {} (length: {}, address: {:?})",
            lit.literal, lit.value, lit.length, lit.address
        ));
    }

    (labeledparsedline, length, startaddr, symbol_table.to_vec())
}
