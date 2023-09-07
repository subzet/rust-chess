// Import piece module
mod piece;
mod board;

fn main() {
    // Create a new board
    let mut board = board::Board::new();

    // Print the board
    for row in board.squares.iter() {
        for square in row.iter() {
            print!("{}", square);
        }
        println!("");
    }

    // Print the board fen string
    println!("{}", board.get_fen_string());
}
