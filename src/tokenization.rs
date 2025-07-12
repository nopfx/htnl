use crate::tokens::Token;

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
    buffer[..buffer.len() - 2].to_string()
}

fn flush_text_buffer(tokens: &mut Vec<Token>, text_buffer: &mut String) {
    if !text_buffer.is_empty() {
        tokens.push(Token::Text(text_buffer.clone()));
        text_buffer.clear();
    }
}

fn handle_directive(directive: &str, tokens: &mut Vec<Token>, text_buffer: &mut String) {
    let trimmed = directive.trim();
    let lower = trimmed.to_lowercase();

    if lower.starts_with("if ") {
        flush_text_buffer(tokens, text_buffer);
        tokens.push(Token::Condition(trimmed[3..].trim().to_string()))
    } else if lower == "else" {
        flush_text_buffer(tokens, text_buffer);
        tokens.push(Token::Else(String::new()));
    } else if lower == "endif" {
        flush_text_buffer(tokens, text_buffer);
        tokens.push(Token::EndIf(String::new()));
    } else if lower.starts_with("for ") {
        flush_text_buffer(tokens, text_buffer);
        tokens.push(Token::ForLoop(trimmed[4..].trim().to_string()));
    } else if lower == "endfor" {
        flush_text_buffer(tokens, text_buffer);
        tokens.push(Token::EndFor(String::new()));
    } else if lower.starts_with("include ") {
        flush_text_buffer(tokens, text_buffer);
        tokens.push(Token::IncludeHTNL(trimmed[8..].trim().to_string()));
    }
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut text_buffer = String::new();

    while let Some(c) = chars.next() {
        if c == '{' {
            if let Some(&next) = chars.peek() {
                match next {
                    '{' => {
                        chars.next();
                        flush_text_buffer(&mut tokens, &mut text_buffer);
                        let var = seek_until(&mut chars, "}}");
                        if !var.is_empty() {
                            tokens.push(Token::Variable(var));
                        }
                    }
                    '%' => {
                        chars.next();
                        let directive = seek_until(&mut chars, "%}");
                        handle_directive(&directive, &mut tokens, &mut text_buffer);
                    }
                    _ => text_buffer.push(c),
                }
            } else {
                text_buffer.push(c);
            }
        } else {
            text_buffer.push(c);
        }
    }

    flush_text_buffer(&mut tokens, &mut text_buffer);
    tokens
}
