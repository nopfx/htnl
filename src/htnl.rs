use std::fs;

pub struct HTNLFile {
    pub path: String,
}

impl HTNLFile {
    pub fn contents(self) -> String {
        fs::read_to_string(self.path).unwrap_or("".into())
    }
}
