use serde::{Deserialize, Serialize};

use crate::models::geometry::square::Square;

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct FfBall {
    square: Square,
    in_game: bool,
    under_control: bool,
    on_ground: bool
}

impl FfBall {
	pub fn new() -> FfBall {
        Default::default()
    }    
}

