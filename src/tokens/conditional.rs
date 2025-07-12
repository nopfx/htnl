pub struct Condition(String);
pub struct EndIf(String);
pub struct Else(String);

pub fn evaluate(expr: &str, context: &std::collections::HashMap<String, String>) -> bool {
    let mut evaluated = String::new();
    for part in expr.split_whitespace() {
        let val = context.get(part).map(|v| v.as_str()).unwrap_or(part);
        evaluated.push_str(val);
        evaluated.push(' ');
    }

    let tokens: Vec<&str> = evaluated.trim().split_whitespace().collect();
    if tokens.len() == 3 {
        match tokens[1] {
            "==" => tokens[0] == tokens[2],
            "!=" => tokens[0] != tokens[2],
            "<=" => {
                let num1: i32 = tokens[0].parse().unwrap_or(0);
                let num2: i32 = tokens[2].parse().unwrap_or(0);
                num1 <= num2
            }
            "<" => {
                let num1: i32 = tokens[0].parse().unwrap_or(0);
                let num2: i32 = tokens[2].parse().unwrap_or(0);
                num1 < num2
            }
            ">=" => {
                let num1: i32 = tokens[0].parse().unwrap_or(0);
                let num2: i32 = tokens[2].parse().unwrap_or(0);
                num1 >= num2
            }
            ">" => {
                let num1: i32 = tokens[0].parse().unwrap_or(0);
                let num2: i32 = tokens[2].parse().unwrap_or(0);
                num1 > num2
            }
            _ => false,
        }
    } else if tokens.len() == 3 && (tokens[1] == "or" || tokens[1] == "and") {
        let left = tokens[0] == "true";
        let right = tokens[2] == "true";
        match tokens[1] {
            "or" => left || right,
            "and" => left && right,
            _ => false,
        }
    } else if tokens.len() == 1 {
        tokens[0] == "true"
    } else {
        false
    }
}
