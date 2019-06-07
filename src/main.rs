// IMPORTS
//
// std
use std::io::{self};

// mods
mod bot;

// STRUCTS
//
// Board struct
// field: 8x8 array
#[derive(Clone)]
pub struct Board {
    field: [usize; 64],
}

// Board implementations
impl Board {
    // Setup board
    fn start() -> Board {
        // Field of board is always 8x8
        let mut field = [0; 64];

        // Standard start layout
        field[27] = 1;
        field[28] = 2;
        field[35] = 2;
        field[36] = 1;

        // Return the Board
        return Board {
            field: field,
        }
    }

    // Create a Board with number values
    // #[dead_code]
    #[cfg(debug_assertions)]
    fn numbers() -> Board {
        // Field of board is always 8x8
        let mut field = [0; 64];
        for i in 0..64 {
            field[i] = i;
        }
        return Board {
            field: field,
        }
    }

    // Print function board
    fn print(&self) {
        // Header: Column letters
        println!("   a b c d e f g h");

        // 8 high
        for i in 0..8 {
            let mut v = String::new();

            // 8 wide
            for j in 0..8 {
                match self.field[(i*8)+j] {
                    0 => v.push_str(". "),
                    1 => v.push_str("o "),
                    2 => v.push_str("* "),
                    _ => panic!("Impossibru field push."),
                };
            }

            // Print row: Row number (top high) and row
            println!("{} [{}]", 8-i, v);
        }
        // Empty line
        println!("");
    }

    // Execute move on board
    // stones:         Move struct
    // opponent_color: color of opponents pieces
    fn execute_move(&mut self, stones: &Move, opponent_color: usize) {
        // Get own color
        let color = match opponent_color {
            1 => 2,
            2 => 1,
            _ => panic!("Impossible opponent color in Board struct."),
        };

        // Set down own stone
        self.field[stones.mv_int] = color;

        // For every vector/line iterate
        for vector in &stones.flips {
            for x in vector {
                // Flip stones
                self.field[*x] = color;
            }
        }
    }

    // Count score of board
    //                  black, white
    fn score(&self) -> (usize, usize) {
        let mut score = (0, 0);
        for i in self.field.iter() {
            match i {
                0 => (),
                1 => score.1 += 1,
                2 => score.0 += 1,
                _ => panic!("Impossibru: counting score: value on field"),
            }
        }
        return score
    }
}

// Simple struct for move
#[derive(Clone,Debug)]
pub struct Move {
    mv_int: usize,
    flips:  Vec<Vec<usize>>,
}

impl Move {
    fn new() -> Move {
        Move {
            mv_int: 0,
            flips: vec![vec![0]],
        }
    }
}


// MAIN
fn main() {
    // Setup
    let mut finished = false;
    let mut board = Board::start();

    // Get max depth
    let depth = depth_input();

    // Start print
    println!("Start:");
    board.print();

    // Game loop
    while ! finished {
        // Player turn (black)
        finished = turn(&mut board, 1);
        board.print();

        // Bot turn (black)
        let bot_move = bot::bot_turn(&mut board, 2, depth);
        board.execute_move(&bot_move, 2);
        // Print the board
        board.print();
    }
}

// Get max depth from input
//
// return:  max_depth usize
fn depth_input() -> usize {
    // While there is no valid input, keep asking
    loop {
        // Get input from stdin
        println!("Give max depth:");
        let mut depth_input = String::new();
        io::stdin().read_line(&mut depth_input).expect("Could not read?");

        // Trim input
        let depth_input_trimmed = depth_input.trim();
        match depth_input_trimmed.parse::<usize>() {
            Ok(n) => {
                return n
            },
            Err(e) => {
                println!("Error: {}", e);
                // On error, try again
                continue
            },
        };
    }
}

// Turn function: Get valid moves and then ask user for move
//  board:  Board Struct
//  color:  Opponent's color
//
//  return: bool; if true, the gameloop ends
fn turn(board: &mut Board, color: usize) -> bool {

    // Get valid moves
    let valid_moves = get_valid_moves(&board, color);
    if valid_moves.len() == 0 {
        println!("No more valid moves.");
        return true
    }

    // Create readable valid moves
    let mut readable_move_list = Vec::new();
    println!("Valid moves:");

    // Parse valid moves to x/y position
    for m in &valid_moves {
        // X position
        let x = match (m.mv_int % 8) + 1 {
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            _ => panic!("Impossibru X value"),
        };
        // Y position (top = 8)
        let y = 8 - (m.mv_int / 8);

        // Create position string
        let pos = format!("{}{}", x, y);

        // Print moves
        println!("{}", pos);

        // Push to readable valid move vec
        readable_move_list.push((pos, m));
    }

    // Get valid input
    let mut valid_input = false;
    while ! valid_input {
        // Get input from stdin
        println!("Chose your move");
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input).expect("Could not read?");

        // Trim input
        let player_input_trimmed = player_input.trim();

        // If choice is in the list of readable valid moves, execute move
        for choice in &readable_move_list {
            // choice.0 = [a1 - h8]
            if (choice.0) == player_input_trimmed {
                // Input was valid
                valid_input = true;
                // Pass valid move (usize, <Vec<Vec<usize>>>)
                board.execute_move(choice.1, color);
                break
            }
        }
    }
    return false
}

// Get valid moves: walk through the board and check what moves are valid
//  board: Board struct
//  color: Opponent's color
//
//  return: A list of tuples;
//          tuple: Valid move with a list of flip-vectors
fn get_valid_moves(board: &Board, color: usize) -> Vec<Move> {
    let mut valid_moves = Vec::new();
    // Iterate over all squares in the field
    for i in 0..64 {
        // If the square isn't empty go to next
        if board.field[i] != 0 {
            continue
        }
        // Get all neighbouring squares; if no opponent piece is found go to next
        let neighbours = check_neighbours(board, i, color);
        if neighbours.len() == 0 {
            continue
        }
        // Check validity of move for neighbouring pieces
        let flips = get_flips(board, &neighbours, i, color);
        if flips.len() == 0 {
            continue
        }
        // Push valid move and flips
        valid_moves.push(
            Move {
                mv_int: i,
                flips: flips,
        });
    }
    // Return all valid moves with flips
    return valid_moves
}

// Check for neighbours
//  board: Board struct
//  pos:   Position on board
//  color: Color of opponent
//
//  return: Vector of neighboring opponents
pub fn check_neighbours(board: &Board, pos: usize, color: usize) -> Vec<usize> {
    let mut neighbours = Vec::new();
    // Check left of position
    // Check if we are not in left column
    if pos != 0 && pos % 8 != 0 {
        if board.field[pos - 1] == color {
            neighbours.push(pos - 1);
        }
    }
    // Check right of position
    // Check if we are not in right colum
    if pos != 63 && (pos + 1) % 8 != 0 {
        if board.field[pos + 1] == color {
            neighbours.push(pos + 1);
        }
    }
    // Check above position
    // Check if we are not on top row
    if pos > 7 {
        if board.field[pos - 8] == color {
            neighbours.push(pos - 8);
        }
        if (pos + 1) % 8 != 0 && board.field[pos - 7] == color {
            neighbours.push(pos - 7);
        }
        if pos % 8 != 0 && board.field[pos - 9] == color {
            neighbours.push(pos - 9);
        }
    }
    // Check below position
    // Check if we're not on bottom row
    if pos < 54 {
        if board.field[pos + 8] == color {
            neighbours.push(pos + 8);
        }
        if pos % 8 != 0&& board.field[pos + 7] == color {
            neighbours.push(pos + 7);
        }
        if (pos + 1) % 8 != 0 && board.field[pos + 9] == color {
            neighbours.push(pos + 9);
        }
    }
    return neighbours
}

// Get targets that will be flipped
//  board:    Board
//  targets:  Neighbouring opponent pieces
//  position: Position on board
//  color:    Oponent color
//
//  return:   List of vectors; vectors are pieces of opponent that'll be flipped
pub fn get_flips(board: &Board, targets: &Vec<usize>, position: usize, opponent_color: usize) -> Vec<Vec<usize>> {
    //
    let color = match opponent_color {
        1 => 2,
        2 => 1,
        _ => 0,
    };

    // Initiate vector
    let mut flips = Vec::new();

    // Iterate through all possible targets
    for t in targets {
        // Position of move
        let mut pos = position as isize;

        // isize target
        let ti = *t as isize;

        // Step; target pos - move pos
        let step = ti - pos;

        // Flip positions
        let mut fp = Vec::new();

        // Loop until we find our own stone, or edge of board
        loop {
            // Walk
            pos = pos + step;

            // Break if we go out of bound
            if (pos + step) < 0 || (pos + step) > 63 {
                break
            }

            // If step is to the right, don't move to next line
            if step == 1 && (pos + step) % 8 == 0 {
                break
            }

            // Do not move to previous line when step is to the left
            if step == -1 && (pos + step) % 8 == 0 {
                break
            }

            // Get color of next position
            let next = board.field[(pos + step) as usize];

            // Check the color of next pos
            //  if 0: Dind't run into own piece
            //  else if opponent's color, push position to flip position vector
            //  Only push flip position vector to actual flips vector when we run into our own piece
            //  then break
            if next == 0 {
                break
            } else if next == opponent_color {
                fp.push(pos as usize);
            } else if next == color {
                fp.push(pos as usize);
                flips.push(fp);
                break
            }
        }
    }
    // Return flips
    return flips
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn botfight() {
        let mut board = Board::start();
        loop {
            let depth = 6;
            let val_moves = get_valid_moves(&board, 1);
            if val_moves.len() == 0 {
                break
            }
            let bot_move = bot::bot_turn(&mut board, 1, depth);
            board.execute_move(&bot_move, 1);
            board.print();
            let val_moves = get_valid_moves(&board, 2);
            let bot_move = bot::bot_turn(&mut board, 2, depth);
            if val_moves.len() == 0 {
                break
            }
            board.execute_move(&bot_move, 2);
            board.print();
        }
        let final_score = board.score();
        println!("Final score: {}-{}", final_score.0, final_score.1);
    }
}
