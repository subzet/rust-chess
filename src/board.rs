// The board is represented by a 8x8 array of Option<Piece>.
// If a square is empty, the corresponding array element is None.
// If a square contains a piece, the corresponding array element is Some(Piece).
// Import piece 
use crate::piece;

#[derive(Clone, Copy)]
pub struct Square {
    pub piece: Option<piece::Piece>,
}

pub struct Board {
    pub squares: [[Square; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        // Create a new board with pieces in their starting positions
        let mut squares = [[Square { piece: None }; 8]; 8];

        // Create white pieces
        squares[7][0] = Square { piece: Some(piece::Piece::new(piece::Color::White, piece::Kind::Rook)) };
        squares[7][1] = Square { piece: Some(piece::Piece::new(piece::Color::White, piece::Kind::Knight)) };
        squares[7][2] = Square { piece: Some(piece::Piece::new(piece::Color::White, piece::Kind::Bishop)) };
        squares[7][3] = Square { piece: Some(piece::Piece::new(piece::Color::White, piece::Kind::Queen)) };
        squares[7][4] = Square { piece: Some(piece::Piece::new(piece::Color::White, piece::Kind::King)) };
        squares[7][5] = Square { piece: Some(piece::Piece::new(piece::Color::White, piece::Kind::Bishop)) };
        squares[7][6] = Square { piece: Some(piece::Piece::new(piece::Color::White, piece::Kind::Knight)) };
        squares[7][7] = Square { piece: Some(piece::Piece::new(piece::Color::White, piece::Kind::Rook)) };
        for i in 0..8 {
            squares[6][i] = Square { piece: Some(piece::Piece::new(piece::Color::White, piece::Kind::Pawn)) };
        }

        // Create black pieces
        squares[0][0] = Square { piece: Some(piece::Piece::new(piece::Color::Black, piece::Kind::Rook)) };
        squares[0][1] = Square { piece: Some(piece::Piece::new(piece::Color::Black, piece::Kind::Knight)) };
        squares[0][2] = Square { piece: Some(piece::Piece::new(piece::Color::Black, piece::Kind::Bishop)) };
        squares[0][3] = Square { piece: Some(piece::Piece::new(piece::Color::Black, piece::Kind::Queen)) };
        squares[0][4] = Square { piece: Some(piece::Piece::new(piece::Color::Black, piece::Kind::King)) };
        squares[0][5] = Square { piece: Some(piece::Piece::new(piece::Color::Black, piece::Kind::Bishop)) };
        squares[0][6] = Square { piece: Some(piece::Piece::new(piece::Color::Black, piece::Kind::Knight)) };
        squares[0][7] = Square { piece: Some(piece::Piece::new(piece::Color::Black, piece::Kind::Rook)) };
        for i in 0..8 {
            squares[1][i] = Square { piece: Some(piece::Piece::new(piece::Color::Black, piece::Kind::Pawn)) };
        }

        return Board { squares };
    }

    pub fn get_fen_string(&self) -> String {
        let mut fen_string = String::new();
        
        for (i, row) in self.squares.iter().enumerate() {
            let mut empty_squares = 0;
            for square in row.iter() {
                match square.piece {
                    Some(ref piece) => {
                        if empty_squares > 0 {
                            fen_string.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen_string.push(piece.get_code_by_color());
                    },
                    None => empty_squares += 1,
                }
            }
            if empty_squares > 0 {
                fen_string.push_str(&empty_squares.to_string());
            }
            // Avoid adding "/" at the end of the last row
            if i < 7 {
                fen_string.push('/');
            }
        }

        fen_string
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.piece {
            Some(ref piece) => write!(f, "{}", piece),
            None => write!(f, "*"),
        }
    }
}

