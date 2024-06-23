use crate::Board;


// self-explanatory
fn count_material(board: &Board) -> i32{
    let white = (board.white_pawns.count_ones() + board.white_bishops.count_ones() * 3 + board.white_knights.count_ones() * 3 + board.white_rooks.count_ones() * 5 + board.white_queens.count_ones() * 9) as i32;
    let black = (board.black_pawns.count_ones() + board.black_bishops.count_ones() * 3 + board.black_knights.count_ones() * 3 + board.black_rooks.count_ones() * 5 + board.black_queens.count_ones() * 9) as i32;
    white - black
}

// does not work yet
fn king_safety(board: &Board) -> i32 {
    let white = board.white_king as i32 * -1 + 65;
    let black = (board.black_king / 10000000000000000) as i32;

    (white - black) / 2
}


/*
Knight position evaluation
Args: &Board
Return: f32

Works by creating quadratic function with arms down (times -1.0) for each color where the peak is:
x: the best position on the board encoded in i64 (d4 and e5 for black; e4 and d5 for white)
y: 2.0 is the reward for the best position on board possible

Since its quadratic im further away the number is from the best one the lower the value.

returns white reward - black reward rounded to 2 digits
*/
fn knight_position(board: &Board) -> f32 {
    let white =  -1.0 * ((board.white_knights as f32) - 68853694464.0).powf(2.0) + 2.0;
    let black =  -1.0 * ((board.black_knights as f32) - 34628173824.0).powf(2.0) + 2.0;
    let mut result = white - black;

    // no idea why but it works, do not change
    let fix = match white > black{
        true => 0,
        false => 1,
    };

    // converts a big ass number into a one that fits in (0,1)
    for _ in 0..(white - black).to_string().chars().count() - fix{
        result = result / 10.0
    }
    (result * 100.0).round() / 100.0
}



// self-explanatory
pub fn evaluation(board: Board) -> f32 {
    let material = count_material(&board);
    // let king = king_safety(&board); not ready
    let knights = knight_position(&board);
    material as f32 + knights
}