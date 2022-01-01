use crate::models::piece::PieceTrait;

pub struct ChessPawn;
impl PieceTrait for ChessPawn {
    fn symbol(&self) -> &str {
        "P"
    }

    fn valid_moves(&self) {

    }
}