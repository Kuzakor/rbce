mod evaluation;

use std::io;


// Main function
fn main() {
    println!("Paste board FEN: ");
    let board = load_position(input().as_str());
    dbg!(&board);
    dbg!(evaluation::evaluation(board));
}

// Board struct
#[derive(Debug)]
struct Board {
    white_pawns: u64,
    white_knights: u64,
    white_bishops: u64,
    white_rooks: u64,
    white_queens: u64,
    white_king: u64,
    black_pawns: u64,
    black_knights: u64,
    black_bishops: u64,
    black_rooks: u64,
    black_queens: u64,
    black_king: u64,
    white_turn: bool,
    white_long_castle: bool,
    white_castle: bool,
    black_long_castle: bool,
    black_castle: bool,
}


/*
Position loading
Args: Forsyth–Edwards Notation of chess board: &str
Return: Board struct

Encodes every location of piece type in u64 integer (since board have 64 squares)
1 means there is a piece, 0 it is not.

*/
fn load_position(fen:&str) -> Board {

    //position

    let mut white_pawns = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let mut white_knights = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let mut white_bishops = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let mut white_rooks = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let mut white_queens = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let mut white_king = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let mut black_pawns = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let mut black_knights = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let mut black_bishops = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let mut black_rooks = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let mut black_queens = String::from("0000000000000000000000000000000000000000000000000000000000000000");
    let mut black_king = String::from("0000000000000000000000000000000000000000000000000000000000000000");

    let mut offset:i32 = 0;
    for (iteration, character) in fen.chars().enumerate(){
        let point = (iteration as i32 + offset) as usize;
        match character {
            'r' => black_rooks.replace_range(point..point + 1, "1"),
            'n' => black_knights.replace_range(point..point + 1, "1"),
            'b' => black_bishops.replace_range(point..point + 1, "1"),
            'q' => black_queens.replace_range(point..point + 1, "1"),
            'k' => black_king.replace_range(point..point + 1, "1"),
            '/' => offset -= 1,
            'p' => black_pawns.replace_range(point..point + 1, "1"),
            'P' => white_pawns.replace_range(point..point + 1, "1"),
            'R' => white_rooks.replace_range(point..point + 1, "1"),
            'N' => white_knights.replace_range(point..point + 1, "1"),
            'B' => white_bishops.replace_range(point..point + 1, "1"),
            'Q' => white_queens.replace_range(point..point + 1, "1"),
            'K' => white_king.replace_range(point..point + 1, "1"),
            ' ' => break,
            _ => offset += character.to_digit(10).unwrap() as i32 - 1
        }
    }

    // conversion to u64
    let white_pawns = bin_to_u64(white_pawns);
    let white_knights = bin_to_u64(white_knights);
    let white_bishops = bin_to_u64(white_bishops);
    let white_rooks = bin_to_u64(white_rooks);
    let white_queens = bin_to_u64(white_queens);
    let white_king = bin_to_u64(white_king);
    let black_pawns = bin_to_u64(black_pawns);
    let black_knights = bin_to_u64(black_knights);
    let black_bishops = bin_to_u64(black_bishops);
    let black_rooks = bin_to_u64(black_rooks);
    let black_queens = bin_to_u64(black_queens);
    let black_king = bin_to_u64(black_king);



    //encoding game data
    let white_turn = fen.contains('w');
    let white_castle = fen.contains(" K");
    let white_long_castle = fen.contains(" Q") || fen.contains(" KQ");
    let black_castle = fen.contains("k ") || fen.contains("kq ");
    let black_long_castle = fen.contains("q ");


    Board {
        white_pawns,
        white_knights,
        white_bishops,
        white_rooks,
        white_queens,
        white_king,
        black_pawns,
        black_knights,
        black_bishops,
        black_rooks,
        black_queens,
        black_king,
        white_turn,
        white_long_castle,
        white_castle,
        black_long_castle,
        black_castle,
    }


}


// self-explanatory
fn bin_to_u64(bin: String) -> u64 {
    u64::from_str_radix(bin.as_str(), 2).unwrap()
}


// self-explanatory
fn input() -> String {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("read error");
    string
}
