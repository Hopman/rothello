use crate::*;

pub fn bot_turn(mut board: Board, color: usize) {
    for valid_moves in get_valid_moves(&board, color) {
        let mut new_board = board.clone();
        new_board.execute_move(valid_moves, color);
        new_board.print();
        bot_turn(new_board, color)
    }
}
