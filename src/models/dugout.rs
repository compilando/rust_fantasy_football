use super::{player::Player, team::Team};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Dugout {
	team: Team,
	reserves: Vec<Player>,
	knocked_out: Vec<Player>,
	dead_and_injured: Vec<Player>
	
}

impl Dugout {
	pub fn new() -> Dugout {
        Default::default()
    }    
}

