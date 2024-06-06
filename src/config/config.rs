#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub max_connections: u32,
    pub timeout: u32,
}

impl Config {
    pub fn new(database_url: &str, max_connections: u32, timeout: u32) -> Self {
        Config {
            database_url: database_url.to_string(),
            max_connections,
            timeout,
        }
    }
}
