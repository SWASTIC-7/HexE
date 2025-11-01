use crate::error::{log_error, log_info};
use crate::predefined::common::SymbolTable;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Token {
    Number(i32),
    Symbol(usize), // Index in symbol table
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
    let expr = expr.trim();

    let mut chars = expr.chars().peekable();

    while let Some(ch) = chars.next() {
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
            ' ' | '\t' => {
                if !current.is_empty() {
                    let mut peek_chars = chars.clone();
                    while let Some(&next_ch) = peek_chars.peek() {
                        if next_ch == ' ' || next_ch == '\t' {
                            peek_chars.next();
                        } else {
                            break;
                        }
                    }

                    if let Some(&next_ch) = peek_chars.peek() {
                        if next_ch == '+'
                            || next_ch == '-'
                            || next_ch == '*'
                            || next_ch == '/'
                            || next_ch == ')'
                        {
                            tokens.push(parse_operand(&current, symbol_table)?);
                            current.clear();
                        }
                    }
                }
            }
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
    let operand = operand.trim();

    if let Ok(num) = operand.parse::<i32>() {
        return Ok(Token::Number(num));
    }

    if let Some(hex_str) = operand
        .strip_prefix("0x")
        .or_else(|| operand.strip_prefix("0X"))
    {
        if let Ok(num) = i32::from_str_radix(hex_str, 16) {
            return Ok(Token::Number(num));
        }
    }

    if let Some(hex_str) = operand.strip_prefix('$') {
        if let Ok(num) = i32::from_str_radix(hex_str, 16) {
            return Ok(Token::Number(num));
        }
    }

    if let Some(index) = symbol_table.iter().position(|sym| sym.label == operand) {
        return Ok(Token::Symbol(index));
    }

    Err(format!("Unknown symbol or invalid number: '{}'", operand))
}

// Evaluating expression using operator precedence (Shunting Yard algorithm)
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

pub fn evaluate_operand_expression(
    expr: &str,
    symbol_table: &[SymbolTable],
) -> Result<u32, String> {
    let tokens = tokenize_expression(expr, symbol_table)?;
    let result = evaluate_expression(tokens, symbol_table)?;

    if result < 0 {
        log_error(&format!(
            "Expression '{}' evaluated to negative value: {}",
            expr, result
        ));
        return Err(format!("Negative result: {}", result));
    }

    log_info(&format!(
        "Expression '{}' = {} (0x{:X})",
        expr, result, result
    ));
    Ok(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expression() {
        let symbols = vec![
            SymbolTable {
                label: "ALPHA".to_string(),
                address: 100,
            },
            SymbolTable {
                label: "BETA".to_string(),
                address: 50,
            },
        ];

        let result = evaluate_operand_expression("ALPHA + BETA", &symbols);
        assert_eq!(result, Ok(150));
    }

    #[test]
    fn test_complex_expression() {
        let symbols = vec![SymbolTable {
            label: "BUF".to_string(),
            address: 10,
        }];

        let result = evaluate_operand_expression("(BUF + 2) * 3", &symbols);
        assert_eq!(result, Ok(36)); // (10 + 2) * 3 = 36
    }
}
