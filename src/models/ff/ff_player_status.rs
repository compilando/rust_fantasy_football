use serde::{Deserialize, Serialize};

use super::ff_turn::FfPlayerTurn;


#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Standing {
	Up,
	Down,
	Stunned
}

impl Default for Standing {
    fn default() -> Standing {
        Standing::Up
    }
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FfPlayerStatus {
    standing: Standing,
	turn: FfPlayerTurn,
	movement_used: i32,
	moved_to_block: bool
}

impl FfPlayerStatus {
	pub fn new() -> FfPlayerStatus {
        FfPlayerStatus {
            standing: Standing::Up,
            turn: FfPlayerTurn::Unused,
            movement_used: 0,
            moved_to_block: false
        }
    }
    
    fn eq(&self, other: &Self) -> bool {
        self.movement_used == other.movement_used
    }
}

