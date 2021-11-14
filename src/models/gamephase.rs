use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct GamePhase {
    name: String,
    turns: i32,
    done: bool
}

impl GamePhase {
	pub fn new(name: String, turns: i32) -> GamePhase {
        GamePhase {
            name,
            turns,
            done: false, 
        }
    }    
}