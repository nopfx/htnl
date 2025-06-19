use crate::tokens::conditional::{Condition, Else, EndIf};
use crate::tokens::text::Text;
use crate::tokens::variable::Variable;

#[derive(Debug)]
pub enum Token {
    Text(String),
    Variable(String),
    Condition(String),
    EndIf(String),
    Else(String),
}

fn seek_until(chars: &mut std::iter::Peekable<std::str::Chars>, end: &str) -> String {
    let mut buffer = String::new();
    while let Some(_) = chars.peek() {
        if buffer.ends_with(end) {
            let len = end.len();
            buffer.truncate(buffer.len() - len);
            return buffer.trim().to_string();
        }
        buffer.push(chars.next().unwrap());
    }
    buffer
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = input.chars().peekable();
    let mut text_buffer = String::new();

    while let Some(c) = chars.next() {
        if c == '{' {
            if let Some(&next) = chars.peek() {
                if next == '{' {
                    chars.next();
                    if !text_buffer.is_empty() {
                        tokens.push(Token::Text(text_buffer.clone()));
                        text_buffer.clear();
                    }
                    let var = seek_until(&mut chars, "}}");
                    if !var.is_empty() {
                        tokens.push(Token::Variable(var));
                    }
                }
                if next == '%' {
                    chars.next();
                    let directive = seek_until(&mut chars, "%}");
                    if directive.trim().starts_with("if ") {
                        if !text_buffer.is_empty() {
                            tokens.push(Token::Text(text_buffer.clone()));
                            text_buffer.clear();
                        }
                        let condition = Token::Condition(directive[3..].to_string());
                        tokens.push(condition);
                    }
                    if directive.trim().starts_with("else") {
                        let elseif = Token::Else(text_buffer.clone());
                        tokens.push(elseif);
                        text_buffer.clear();
                    }
                    if directive.trim().starts_with("endif") {
                        let endif = Token::EndIf(text_buffer.clone());
                        tokens.push(endif);
                        text_buffer.clear();
                    }
                }
            }
        } else {
            text_buffer.push(c);
        }
    }
    if !text_buffer.is_empty() {
        tokens.push(Token::Text(text_buffer));
    }
    return tokens;
}
