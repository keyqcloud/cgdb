use std::collections::HashMap;

#[derive(Debug)]
pub struct Context {
    pub id: u32,
    pub name: String,
    pub properties: HashMap<String, String>,
}

impl Context {
    pub fn new(id: u32, name: &str) -> Self {
        Context {
            id,
            name: name.to_string(),
            properties: HashMap::new(),
        }
    }

    pub fn add_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }

    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}
