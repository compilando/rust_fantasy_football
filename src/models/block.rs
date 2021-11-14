use super::player::Player;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum BlockSum {
	AttackerDoubleStrong, 
	AttackerStronger,
	Equal,
	DefenderStronger,
	DefenderDoubleStrong
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Block {
    attacker: Player,
	defender: Player
}

impl Block {
	pub fn new() -> Block {
        Default::default()
    }    
}

