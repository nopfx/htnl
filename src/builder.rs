use crate::tokenization::{Token, tokenize};
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
                Token::ForLoop(varname) => {
                    let mut loop_block = Vec::new();
                    while let Some(t) = tokens.next() {
                        if matches!(t, Token::EndFor(_)) {
                            break;
                        }
                        loop_block.push(t);
                    }

                    let items = self.loop_items(&varname);
                    for item in items {
                        for token in &loop_block {
                            match token {
                                Token::Text(txt) => html.push_str(txt),
                                Token::Variable(var) => {
                                    let val = item
                                        .get(var)
                                        .or_else(|| self.context.get(var))
                                        .cloned()
                                        .unwrap_or_else(String::new);
                                    html.push_str(&val);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Token::IncludeHTNL(i) => {
                    let template: String = String::from(i.trim());
                    let htnlfile = HTNLFile { path: template };
                    let c = htnlfile.contents();

                    let include_data = Builder {
                        context: self.context.clone(),
                        content: String::from(c),
                    };
                    html.push_str(&include_data.build());
                }
                _ => {}
            };
        }
        html
    }
    fn loop_items(&self, loop_name: &str) -> Vec<HashMap<String, String>> {
        let mut grouped: HashMap<usize, HashMap<String, String>> = HashMap::new();

        for (key, value) in &self.context {
            if let Some(stripped) = key.strip_prefix(&format!("{loop_name}.")) {
                if let Some(rest) = stripped.strip_prefix('[') {
                    if let Some((idx_str, field_path)) = rest.split_once("].") {
                        if let Ok(idx) = idx_str.parse::<usize>() {
                            grouped
                                .entry(idx)
                                .or_insert_with(HashMap::new)
                                .insert(field_path.to_string(), value.clone());
                        }
                    }
                }
            }
        }

        let mut items: Vec<(usize, HashMap<String, String>)> = grouped.into_iter().collect();
        items.sort_by_key(|(i, _)| *i);
        items.into_iter().map(|(_, map)| map).collect()
    }
}
