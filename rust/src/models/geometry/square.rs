use serde::{Deserialize, Serialize};

use crate::models::piece::{Piece, PieceTrait};

#[derive(Default, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Square {
    x: i32,
    y: i32,
    pub piece: Option<Box<Piece>>
}

impl Square {
    pub fn new() -> Square {
        Square { piece: None, x: 0, y: 0 }
    }

    pub fn symbol(&self) -> &str {
        match self.piece {
            Some(ref p) => p.symbol(),
            None => "."
        }
    }	
}

