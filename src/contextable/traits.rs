use std::collections::HashMap;

pub trait Contextable {
    fn flatten(&self) -> HashMap<String, String>;
}

impl<T: Contextable> Contextable for Vec<T> {
    fn flatten(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();

        for (i, item) in self.iter().enumerate() {
            let inner_map = item.flatten();
            for (k, v) in inner_map {
                map.insert(format!("{}.{}", i, k), v);
            }
        }

        map
    }
}
