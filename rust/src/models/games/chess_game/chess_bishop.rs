use crate::models::piece::PieceTrait;

pub struct ChessBishop;
impl PieceTrait for ChessBishop {
    fn symbol(&self) -> &str {
        "P"
    }

    fn valid_moves(&self) {

    }
}