use crate::Board;


// self-explanatory
fn count_material(board: &Board) -> i32{
    let white = (board.white_pawns.count_ones() + board.white_bishops.count_ones() * 3 + board.white_knights.count_ones() * 3 + board.white_rooks.count_ones() * 5 + board.white_queens.count_ones() * 9) as i32;
    let black = (board.black_pawns.count_ones() + board.black_bishops.count_ones() * 3 + board.black_knights.count_ones() * 3 + board.black_rooks.count_ones() * 5 + board.black_queens.count_ones() * 9) as i32;
    white - black
}

// Converts a big number to one that's less than 10.
fn shrink(num: &u64) -> f32 {
    let mut num = num.clone() as f32;
    for _ in 0..num.to_string().chars().count(){
        num = num / 10.0
    }
    num
}

/*
Works by creating quadratic function with arms down (times -1.0) for each color where the peak is:
x: the best position on the board encoded in i64 and shrunk to decimal (d4 and e5 for black; e4 and d5 for white in case of knights)
y: 100.0 is the reward for the best position on board possible

Since its quadratic the further away the number is from the best one the lower the value.

Good enough for knights and king

returns (white reward - black reward) rounded up to 2 digits
*/

fn piece_position_quadratic(pieces_white: &u64, pieces_black: &u64, optimal_white: &u64, optimal_black: &u64) -> f32 {
    let white =  -1.0 * (shrink(pieces_white) - shrink(optimal_white)).powf(2.0) + 100.0;
    let black =  -1.0 * (shrink(pieces_black) - shrink(optimal_black)).powf(2.0) + 100.0;
    let mut result = white - black;
    (result * 100.0).round() / 50.0

}



// I HAVE NO FUCKING CLUE HOW BUT IT WORKS KINDA FINE IMO, I cannot explain it anyway
//const PI:f32 = std::f32::consts::FRAC_PI_2;

fn piece_location_sinus(pieces_white: &u64, pieces_black: &u64) -> f32 {
    let white = f32::sin(pieces_white.clone() as f32) + 0.51;
    let black = f32::sin(pieces_black.clone() as f32) + 0.25;
    dbg!(black);
    (white - black * 100.0).round() / 100.0
}


// self-explanatory
pub fn evaluation(board: Board) -> f32 {
    let material = count_material(&board);
    let knights = piece_position_quadratic(&board.white_knights, &board.black_knights, &68853694464, &34628173824);
    let king = piece_position_quadratic(&board.white_king, &board.black_king, &1, &72057594037927936);
    //let bishops = piece_location_sinus(&board.white_bishops, &board.black_bishops);
    //dbg!(bishops)

    material as f32 + knights + king
}