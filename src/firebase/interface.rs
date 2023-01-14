use std::collections::HashMap;

pub struct FirebaseInterface {
    hash_map: HashMap<String, String>,
}

impl FirebaseInterface {
    pub fn new(hash_map: HashMap<String, String>) -> Self {
        Self { hash_map }
    }
}
