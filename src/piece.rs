//use crate::board;

#[derive(Clone, Copy)]
pub enum Kind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
}

pub struct Move {
    pub piece: Piece,
    pub from: (usize, usize),
    pub to: (usize, usize),
}

impl Piece {
    pub fn new(color: Color, kind: Kind) -> Piece {
        Piece { color, kind }
    }

    pub fn get_code_by_color(&self) -> char {
        match (self.color, self.kind) {
            (Color::White, Kind::Pawn) => 'P',
            (Color::White, Kind::Knight) => 'N',
            (Color::White, Kind::Bishop) => 'B',
            (Color::White, Kind::Rook) => 'R',
            (Color::White, Kind::Queen) => 'Q',
            (Color::White, Kind::King) => 'K',
            (Color::Black, Kind::Pawn) => 'p',
            (Color::Black, Kind::Knight) => 'n',
            (Color::Black, Kind::Bishop) => 'b',
            (Color::Black, Kind::Rook) => 'r',
            (Color::Black, Kind::Queen) => 'q',
            (Color::Black, Kind::King) => 'k',
        }
    }

    pub fn get_score(&self) -> i32 {
        match self.kind {
            Kind::Pawn => 1,
            Kind::Knight => 3,
            Kind::Bishop => 3,
            Kind::Rook => 5,
            Kind::Queen => 9,
            Kind::King => 0,
        }
    }

    // // Get the possible moves for the piece
    // pub fn get_moves(board: board::Board) -> Vec<Move> {
    //     match self.kind {
    //         Kind::Pawn => self.get_pawn_moves(board),
    //         Kind::Knight => self.get_knight_moves(board),
    //         Kind::Bishop => self.get_bishop_moves(board),
    //         Kind::Rook => self.get_rook_moves(board),
    //         Kind::Queen => self.get_queen_moves(board),
    //         Kind::King => self.get_king_moves(board),
    //     }
    // }

    // fn get_pawn_moves(board: board::Board) -> Vec<Move> {
    //     // TODO Implement
    // }

    // fn get_knight_moves(board: board::Board) -> Vec<Move> {
    //     // TODO Implement
    // }

    // fn get_bishop_moves(board: board::Board) -> Vec<Move> {
    //     // TODO Implement
    // }

    // fn get_rook_moves(board: board::Board) -> Vec<Move> {
    //     // TODO Implement
    // }

    // fn get_queen_moves(board: board::Board) -> Vec<Move> {
    //     // TODO Implement
    // }

    // fn get_king_moves(board: board::Board) -> Vec<Move> {
    //     // TODO Implement
    // }
}


impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Piece { color: Color::White, kind: Kind::Pawn } => write!(f, "♙"),
            Piece { color: Color::White, kind: Kind::Knight } => write!(f, "♘"),
            Piece { color: Color::White, kind: Kind::Bishop } => write!(f, "♗"),
            Piece { color: Color::White, kind: Kind::Rook } => write!(f, "♖"),
            Piece { color: Color::White, kind: Kind::Queen } => write!(f, "♕"),
            Piece { color: Color::White, kind: Kind::King } => write!(f, "♔"),
            Piece { color: Color::Black, kind: Kind::Pawn } => write!(f, "♟"),
            Piece { color: Color::Black, kind: Kind::Knight } => write!(f, "♞"),
            Piece { color: Color::Black, kind: Kind::Bishop } => write!(f, "♝"),
            Piece { color: Color::Black, kind: Kind::Rook } => write!(f, "♜"),
            Piece { color: Color::Black, kind: Kind::Queen } => write!(f, "♛"),
            Piece { color: Color::Black, kind: Kind::King } => write!(f, "♚"),
        }
    }
}


