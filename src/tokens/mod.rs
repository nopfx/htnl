pub mod conditional;
pub mod include;
pub mod loops;
pub mod text;
pub mod variable;

#[derive(Debug, Clone)]
pub enum Token {
    Text(String),
    Variable(String),
    Condition(String),
    EndIf(String),
    Else(String),
    IncludeHTNL(String),
    ForLoop(String),
    EndFor(String),
}
