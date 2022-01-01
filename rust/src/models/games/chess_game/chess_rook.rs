use crate::models::piece::PieceTrait;

pub struct ChessRook;
impl PieceTrait for ChessRook {
    fn symbol(&self) -> &str {
        "R"
    }

    fn valid_moves(&self) {

    }
}