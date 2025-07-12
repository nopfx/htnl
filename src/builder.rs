use crate::tokenization::tokenize;
use crate::tokens::Token;
use std::collections::HashMap;

use crate::htnl::HTNLFile;
use crate::tokens::conditional;

pub type Context = HashMap<String, String>;
pub struct Builder {
    pub context: Context,
    pub content: String,
}

impl Builder {
    pub fn build(&self) -> String {
        let mut tokens = tokenize(&self.content).into_iter().peekable();
        let mut html = String::new();

        while let Some(token) = tokens.next() {
            match token {
                Token::Text(txt) => html.push_str(&txt),
                Token::Variable(var) => html.push_str(self.get_variable(&var).as_str()),
                Token::Condition(c) => self.handle_condition(&mut tokens, &c, &mut html),
                Token::ForLoop(var_name) => self.handle_loop(&mut tokens, &var_name, &mut html),
                Token::IncludeHTNL(path) => html.push_str(&self.handle_include(&path)),
                _ => {}
            }
        }
        html
    }
    fn get_variable(&self, var: &str) -> String {
        self.context.get(var).cloned().unwrap_or_default()
    }
    fn handle_condition<I: Iterator<Item = Token>>(
        &self,
        tokens: &mut std::iter::Peekable<I>,
        condition: &str,
        html: &mut String,
    ) {
        let is_true = conditional::evaluate(condition, &self.context);

        let mut then_block = String::new();
        let mut else_block = String::new();
        let mut collecting_else = false;

        while let Some(t) = tokens.peek() {
            match t {
                Token::Else(_) => {
                    tokens.next();
                    collecting_else = true;
                }
                Token::EndIf(_) => {
                    tokens.next();
                    break;
                }
                _ => {
                    let token = tokens.next().unwrap();
                    let target = if collecting_else {
                        &mut else_block
                    } else {
                        &mut then_block
                    };
                    self.render_token(&token, target);
                }
            }
        }

        if is_true {
            html.push_str(&then_block);
        } else {
            html.push_str(&else_block);
        }
    }
    fn render_token(&self, token: &Token, output: &mut String) {
        match token {
            Token::Text(txt) => output.push_str(txt),
            Token::Variable(var) => output.push_str(&self.get_variable(var)),
            _ => {} // ignore non-text tokens inside partials
        }
    }

    fn loop_items(&self, var_name: &str) -> Vec<HashMap<String, String>> {
        let mut grouped: HashMap<usize, HashMap<String, String>> = HashMap::new();

        for (key, value) in &self.context {
            if let Some(stripped) = key.strip_prefix(&format!("{var_name}.")) {
                if let Some(rest) = stripped.strip_prefix('[') {
                    if let Some((idx_str, field)) = rest.split_once("].") {
                        if let Ok(index) = idx_str.parse::<usize>() {
                            grouped
                                .entry(index)
                                .or_insert_with(HashMap::new)
                                .insert(field.to_string(), value.clone());
                        }
                    }
                }
            }
        }

        let mut items: Vec<_> = grouped.into_iter().collect();
        items.sort_by_key(|(i, _)| *i);
        items.into_iter().map(|(_, map)| map).collect()
    }

    fn handle_loop<I: Iterator<Item = Token>>(
        &self,
        tokens: &mut std::iter::Peekable<I>,
        var_name: &str,
        html: &mut String,
    ) {
        let mut loop_tokens = Vec::new();
        while let Some(t) = tokens.next() {
            if matches!(t, Token::EndFor(_)) {
                break;
            }
            loop_tokens.push(t);
        }

        for item in self.loop_items(var_name) {
            for token in &loop_tokens {
                match token {
                    Token::Text(txt) => html.push_str(txt),
                    Token::Variable(var) => {
                        let val = item
                            .get(var)
                            .or_else(|| self.context.get(var))
                            .cloned()
                            .unwrap_or_default();
                        html.push_str(&val);
                    }
                    _ => {}
                }
            }
        }
    }

    fn handle_include(&self, path: &str) -> String {
        let file = HTNLFile {
            path: path.trim().to_string(),
        };
        let contents = file.contents();

        let builder = Builder {
            // I dont like this (need to think something smarter)
            context: self.context.clone(),
            content: contents,
        };
        builder.build()
    }
}
