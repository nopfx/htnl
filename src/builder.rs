use crate::tokenization::{Token, tokenize};
use std::collections::HashMap;

use crate::tokens::conditional;

pub type Context = HashMap<String, String>;
pub struct Builder {
    pub context: Context,
    pub content: String,
}

impl Builder {
    pub fn build(&self) -> String {
        let mut tokens = tokenize(self.content.as_ref()).into_iter().peekable();
        let mut html = String::new();
        while let Some(token) = tokens.next() {
            match token {
                Token::Text(txt) => html.push_str(&*txt),
                Token::Variable(var) => {
                    let v = self.context.get(&var).unwrap_or(&String::new()).clone();
                    html.push_str(&*v);
                }
                Token::Condition(c) => {
                    let cond = conditional::evaluate(&c, &self.context);
                    if cond {
                        match tokens.next() {
                            Some(Token::Else(e)) => html.push_str(&e.trim()),
                            Some(Token::EndIf(e)) => html.push_str(&e.trim()),
                            _ => {}
                        };
                    } else {
                        tokens.next();
                        if matches!(tokens.peek(), Some(Token::Else(_))) {
                            if let Some(Token::Else(e)) = tokens.next() {
                                html.push_str(&e.trim());
                            }
                        }
                        if matches!(tokens.peek(), Some(Token::EndIf(_))) {
                            if let Some(Token::EndIf(e)) = tokens.next() {
                                html.push_str(&e.trim());
                            }
                        }
                    }
                }
                _ => {}
            };
        }
        html
    }
}
