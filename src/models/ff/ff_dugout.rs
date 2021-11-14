
use serde::{Deserialize, Serialize};

use crate::models::{piece::Piece, team::Team};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FfDugout {
	team: Team,
	reserves: Vec<Piece>,
	knocked_out: Vec<Piece>,
	dead_and_injured: Vec<Piece>
	
}

impl FfDugout {
	pub fn new() -> FfDugout {
        Default::default()
    }    
}

