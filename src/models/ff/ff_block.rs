use serde::{Deserialize, Serialize};

use crate::models::piece::Piece;


#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum FfBlockSum {
	AttackerDoubleStrong, 
	AttackerStronger,
	Equal,
	DefenderStronger,
	DefenderDoubleStrong
}

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FfBlock {
    attacker: Piece,
	defender: Piece
}

impl FfBlock {
	pub fn new() -> FfBlock {
        Default::default()
    }    
}

