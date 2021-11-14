use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Square {
    x: i32,
    y: i32,
}

impl Square {
	pub fn new() -> Square {
        Default::default()
    }    
}

