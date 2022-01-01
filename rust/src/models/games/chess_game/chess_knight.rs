use crate::models::piece::PieceTrait;

pub struct ChessKnight;
impl PieceTrait for ChessKnight {
    fn symbol(&self) -> &str {
        "I"
    }

    fn valid_moves(&self) {

    }
}