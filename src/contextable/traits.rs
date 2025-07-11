use std::collections::HashMap;

pub trait Contextable {
    fn flatten(&self) -> HashMap<String, String>;
}

// String and primitives
impl Contextable for String {
    fn flatten(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("".to_string(), self.clone());
        map
    }
}

impl Contextable for i32 {
    fn flatten(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("".to_string(), self.to_string());
        map
    }
}

// Recursive flatten for Vec<T>
impl<T: Contextable> Contextable for Vec<T> {
    fn flatten(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for (i, item) in self.iter().enumerate() {
            let inner_map = item.flatten();
            for (k, v) in inner_map {
                // key like: [0].field or [0] if no key
                if k.is_empty() {
                    map.insert(format!("[{}]", i), v);
                } else {
                    map.insert(format!("[{}].{}", i, k), v);
                }
            }
        }
        map
    }
}
