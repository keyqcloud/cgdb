#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    // Add other configuration fields
}

impl Config {
    pub fn new(database_url: String) -> Self {
        Config {
            database_url,
            // Initialize other fields
        }
    }
}
