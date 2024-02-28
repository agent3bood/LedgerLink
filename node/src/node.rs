pub struct Node {
    id: String,
    key: String,
    neighbors: Vec<String>,
}

impl Node {
    pub fn new(id: String, key: String) -> Node {
        Node {
            id,
            key,
            neighbors: Vec::new(),
        }
    }
}
