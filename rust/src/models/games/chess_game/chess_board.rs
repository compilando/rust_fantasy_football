use crate::models::{games::chess_game::{chess_bishop::ChessBishop, chess_king::ChessKing, chess_knight::ChessKnight, chess_pawn::ChessPawn, chess_queen::ChessQueen, chess_rook::ChessRook}, geometry::square::Square};

pub struct ChessBoard {
    name: String,
    squares: [[Square; 8]; 8]
}

impl ChessBoard {
	pub fn new(name: String) -> ChessBoard {

        let mut squares = [[Square { piece: None, x: 0, y: 0 }; 8]; 8];

        for x in 0..8 {
            squares[x][1].piece = Some(ChessPawn);
            squares[x][6].piece = Some(ChessPawn);
            let piece = match x {
                0 | 7 => Some(ChessRook),
                1 | 6 => Some(ChessKnight),
                2 | 5 => Some(ChessBishop),
                3 => Some(ChessKing),
                4 => Some(ChessQueen),
                _ => unreachable!()
            };
            squares[x][0].piece = piece;
            squares[x][7].piece = piece;
        }

        ChessBoard {
            name: name,
            squares: squares
        }
    }

    impl fmt::Display for ChessBoard {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut res = String::new();
            for i in 0..8 {
                for j in 0..8 {
                    res.push_str(self.squares[(i * 8) + j].symbol());
                }
                res.push_str("\n");
            }
            write!(f, "{}", res)
        }
    }
}