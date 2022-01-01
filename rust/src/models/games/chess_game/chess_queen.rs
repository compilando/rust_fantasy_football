use crate::models::piece::PieceTrait;

pub struct ChessQueen;
impl PieceTrait for ChessQueen {
    fn symbol(&self) -> &str {
        "Q"
    }

    fn valid_moves(&self) {

    }
}