// shared
//
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub this: f32,
    pub is: f32,
    pub a: u8,
    pub _test: i32,
}
