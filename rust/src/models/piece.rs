use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Piece {

}

pub trait PieceTrait {
    fn symbol(&self) -> &str;
    fn valid_moves(&self);
}

impl Piece {
	pub fn new() -> Piece {
        Default::default()
    }
}

impl PieceTrait for Piece {
    fn symbol(&self) -> &str {
        ""
    }

    fn valid_moves(&self) {
        
    }
}

