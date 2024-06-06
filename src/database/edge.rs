pub struct Edge {
    from: u32,
    to: u32,
    // Add other fields as needed
}

impl Edge {
    pub fn new(from: u32, to: u32) -> Self {
        Edge {
            from,
            to,
            // Initialize other fields
        }
    }
}
