use std::{
    collections::HashMap
};
use log::{ info };

//1. Main Struct
#[derive(Debug)]
pub struct KeyValue {
    pairs: HashMap<String, String>,
    value: i64
}

//2. Default Implementation
impl Default for KeyValue {
    fn default() -> Self {
        Self {
            pairs: HashMap::new(),
            value: 0
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

    pub fn get_val(self) -> i64 {
        self.value
    }

    pub fn increment(&mut self) -> Option<i64> {
        self.value += 1;
        env_logger::init();
        let log_message = format!("Increased number to {}", &self.value);
        info!("{:?}", log_message.as_bytes());
        after_counter_change();

        Some(self.value)
    }

    pub fn decrement(&mut self) -> Option<i64> {
        self.value -= 1;
        let log_message = format!("Increased number to {}", &self.value);
        info!("{:?}", log_message.as_bytes());
        after_counter_change();

        Some(self.value)
    }

    pub fn reset(&mut self) -> Option<i64> {
        self.value = 0;
        let log_message = format!("Reset counter to zero");
        info!("{:?}", log_message.as_bytes());

        Some(self.value)
    }
}

fn after_counter_change() {
    // show helpful warning that i8 (8-bit signed integer) will overflow above 127 or below -128
    info!("{:?}", "Make sure you don't overflow, my friend.".as_bytes());
}