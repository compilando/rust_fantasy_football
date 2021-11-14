use super::square::Square;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Ball {
    square: Square,
    in_game: bool,
    under_control: bool,
    on_ground: bool
}

impl Ball {
	pub fn new() -> Ball {
        Default::default()
    }    
}

