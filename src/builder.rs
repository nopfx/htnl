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
                        if let Some(Token::Else(cond_true)) = tokens.next() {
                            html.push_str(&cond_true.trim());
                        }
                    } else {
                        tokens.next(); // shift true
                        if let Some(Token::EndIf(cond_false)) = tokens.next() {
                            html.push_str(&cond_false.trim());
                        }
                    }
                }
                _ => {}
            };
        }
        html
    }
}
