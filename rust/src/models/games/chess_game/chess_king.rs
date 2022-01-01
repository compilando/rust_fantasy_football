use crate::models::piece::PieceTrait;

pub struct ChessKing;
impl PieceTrait for ChessKing {
    fn symbol(&self) -> &str {
        "K"
    }

    fn valid_moves(&self) {

    }
}