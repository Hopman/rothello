// IMPORTS
//
// std
use std::io::{self};

// mods
//mod bot;

// STRUCTS
//
// Board struct
// field: 8x8 array
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
        return Board {
            field: field,
        }
    }

    // Print function board
    fn print(&self) {
        // 8 high
        for i in 0..8 {
            let mut v = Vec::new();
            // 8 wide
            for j in 0..8 {
                v.push(&self.field[(i*8)+j]);
            }
            // Print row
            println!("{:?}", v);
        }
        // Empty line
        println!("");
    }

    // Execute move on board
    // stones:         tuple of possition and flip vectors
    // opponent_color: color of opponents pieces
    fn execute_move(&mut self, stones: &(usize, Vec<Vec<usize>>), opponent_color: usize) {
        // Get own color
        let color = match opponent_color {
            1 => 2,
            2 => 1,
            _ => panic!("Impossible opponent color in Board struct."),
        };

        // Set down own stone
        self.field[stones.0] = color;

        // For every vector/line iterate
        for vector in &stones.1 {
            for x in vector {
                // Flip stones
                self.field[*x] = color;
            }
        }
    }

}

// MAIN
fn main() {
    // Setup
    let mut finished = false;
    let mut board = Board::start();
    let mut steps = 0;

    // Start print
    println!("Start:");
    board.print();

    // Game loop
    while ! finished {
        // There's never more than 60 steps
        if steps >= 60 {
            finished = true;
        }
        println!("Turn: {}", steps);

        // Player turn (white)
        turn(&mut board, 2);
        steps += 1;
        board.print();

        // Player turn (black)
        turn(&mut board, 1);
        steps += 1;
        board.print();
    }
}

// Turn function: Get valid moves and then ask user for move
//  board: Board Struct
//  color: Opponent's color
//
//  return nothing: Board is mutated in struct
fn turn(mut board: &mut Board, color: usize) {

    // Get valid moves
    let valid_moves = get_valid_moves(&board, color);
    if valid_moves.len() == 0 {
        println!("No more valid moves.");
        return
    }

    // Print valid moves
    println!("Valid moves:");
    for m in &valid_moves {
        println!("{}", m.0);
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

        // Try casting to int
        let player_input_int = match player_input_trimmed.parse::<usize>() {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                // On error, try again
                continue
            },
        };

        // If choice is in the list of moves, execute move
        for choice in &valid_moves {
            if choice.0 == player_input_int {
                // Input was valid
                valid_input = true;
                board.execute_move(choice, color);
                break
            }
        }
    }
}

// Get valid moves: walk through the board and check what moves are valid
//  board: Board struct
//  color: Opponent's color
//
//  return: A list of tuples;
//          tuple: Valid move with a list of flip-vectors
fn get_valid_moves(board: &Board, color: usize) -> Vec<(usize, Vec<Vec<usize>>)> {
    let mut valid_moves = Vec::new();
    // Iterate over all squares in the field
    for i in 0..64 {
        // If the square isn't empty continue
        if board.field[i] != 0 {
            continue
        }
        // Get all neighbouring squares; if an opponent piece is found; continue
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
        valid_moves.push((i, flips));
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
    if pos > 8 {
        if board.field[pos - 8] == color {
            neighbours.push(pos - 8);
        }
        if pos % 8 != 0 {
            if board.field[pos - 7] == color {
                neighbours.push(pos - 7);
            }
        }
        if (pos + 1) % 8 != 0 {
            if board.field[pos - 9] == color {
                neighbours.push(pos - 9);
            }
        }
    }
    // Check below position
    // Check if we're not on bottom row
    if pos < 56 {
        if board.field[pos + 8] == color {
            neighbours.push(pos + 8);
        }
        if pos % 8 != 0 {
            if board.field[pos + 7] == color {
                neighbours.push(pos + 7);
            }
        }
        if (pos + 1) % 8 != 0 {
            if board.field[pos + 9] == color {
                neighbours.push(pos + 9);
            }
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
        // Position
        let mut pos = position as isize;
        // Isize target
        let ti = *t as isize;
        // Step
        let step = ti - pos;
        // Flip positions
        let mut fp = Vec::new();

        // Loop until we find our own stone, or edge of board
        loop {
            // Walk
            pos = pos + step;

            // Break if we find edge or empty square
            if (pos + step) < 0 || (pos + step) > 63  ||
                pos % 8 == 0    || (pos + 1) % 8 == 0 {
                    break
                }

            // Get color of next position
            let next = board.field[(pos + step) as usize];

            // Check the color of next pos
            // If 0: Dind't run into own piece
            if next == 0 {
                break
            // If opponent's color, push position to flip position vector
            } else if next == opponent_color {
                fp.push(pos as usize);
            // Only push flip position vector to actual flips vector when we run into our own piece
            // then break
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
