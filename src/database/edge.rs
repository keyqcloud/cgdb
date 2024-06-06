#[derive(Debug)]
pub struct Edge {
    pub from: u32,
    pub to: u32,
    pub label: String,
    pub properties: std::collections::HashMap<String, String>,
}

impl Edge {
    pub fn new(from: u32, to: u32, label: &str) -> Self {
        Edge {
            from,
            to,
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
