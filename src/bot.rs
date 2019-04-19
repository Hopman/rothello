use crate::*;

// RECURSIVE FUNCTION
// It _actually_ works
pub fn bot_turn(mut board: Board, color: usize) {
    for valid_move in get_valid_moves(&board, color) {
        let mut new_board = board.clone();
        new_board.execute_move(&valid_move, color);
        new_board.print();
        let new_color = match color {
            1 => 2,
            2 => 1,
            _ => panic!("Impossibru!"),
        };
        bot_turn(new_board, new_color);
    }
}
