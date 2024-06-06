#[derive(Debug)]
pub struct Node {
    pub id: u32,
    pub label: String,
    pub properties: std::collections::HashMap<String, String>,
}

impl Node {
    pub fn new(id: u32, label: &str) -> Self {
        Node {
            id,
            label: label.to_string(),
            properties: std::collections::HashMap::new(),
        }
    }

    pub fn add_property(&mut self, key: &str, value: &str) {
        self.properties.insert(key.to_string(), value.to_string());
    }

    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}
