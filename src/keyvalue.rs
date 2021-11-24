use std::{
    collections::HashMap
};

//1. Main Struct
#[derive(Debug)]
pub struct KeyValue {
    pairs: HashMap<String, String>
}

//2. Default Implementation
impl Default for KeyValue {
    fn default() -> Self {
        Self {
            pairs: HashMap::new()
        }
    }
}

//3. Core Logic
impl KeyValue {
    pub fn create_update(&mut self, k: String, v: String) {
        self.pairs.insert(k, v);
    }

    pub fn read(&self, k: String) -> Option<&String> {
        self.pairs.get(&k)
    }

    pub fn delete(&mut self, k: String) {
        self.pairs.remove(&k);
    }
}