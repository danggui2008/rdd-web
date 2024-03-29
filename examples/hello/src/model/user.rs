use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize,Debug)]
pub struct User {
    name: String,
    age: u8,
}

impl User {
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}
