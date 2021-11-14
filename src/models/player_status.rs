use serde::{Deserialize, Serialize};

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

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum PlayerTurn {
	Unused,
	MoveAction,
	PassAction,
	BlitzAction,
	HandOffAction,
	FoulAction,
	BlockAction,
	Used
}
impl Default for PlayerTurn {
    fn default() -> PlayerTurn {
        PlayerTurn::Unused
    }
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct PlayerStatus {
    standing: Standing,
	turn: PlayerTurn,
	movement_used: i32,
	moved_to_block: bool
}

impl PlayerStatus {
	pub fn new() -> PlayerStatus {
        PlayerStatus {
            standing: Standing::Up,
            turn: PlayerTurn::Unused,
            movement_used: 0,
            moved_to_block: false
        }
    }
    
    fn eq(&self, other: &Self) -> bool {
        self.movement_used == other.movement_used
    }
}

