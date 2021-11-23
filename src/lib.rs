use borsh::{
    ser::BorshSerialize,
    de::BorshDeserialize
};

#[derive(Default)]
pub struct Counter {
    val: i64,
}

impl Counter {
    pub fn get_num(&self) -> i64 {
        self.val
    }

    pub fn increment(&mut self) {
        self.val += 1;
        
    }
}