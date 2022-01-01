use serde::{Deserialize, Serialize};

use super::player::Player;

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct GamePhase {
    name: String,
    turns: i32,
    done: bool,
    player: Option<Player>
}

impl GamePhase {
	pub fn new(name: String, turns: i32) -> GamePhase {
        GamePhase {
            name,
            turns,
            done: false, 
            player: None, 
        }
    }    
}