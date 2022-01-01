use super::{team::Team};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Player {
    name: String,
	team: Team
}

impl Player {
	pub fn new() -> Player {
        Default::default()
    }    
}

