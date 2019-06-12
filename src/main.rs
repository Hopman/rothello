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
    field: [Disk; 64],
}

// Board implementations
impl Board {
    // Setup board
    fn start() -> Board {
        // Field of board is always 8x8
        let mut field = [Disk::Empty; 64];

        // Standard start layout
        // White start positions
        field[27] = Disk::White;
        field[36] = Disk::White;

        // Black start positions
        field[28] = Disk::Black;
        field[35] = Disk::Black;

        // Return the Board
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
                    Disk::White => v.push_str("o "),
                    Disk::Black => v.push_str("* "),
                    _ => v.push_str(". "),
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
    fn execute_move(&mut self, stones: &Move, player: Player) {
        // Set down own stone
        self.field[stones.mv_int] = player.color;

        // For every vector/line iterate
        for vector in &stones.flips {
            for x in vector {
                // Flip stones
                self.field[*x] = player.color;
            }
        }
    }

    // Count score of board
    //                  black, white
    fn score(&self) -> (usize, usize) {
        let mut score = (0, 0);
        for i in self.field.iter() {
            match i {
                Disk::Black => score.0 += 1,
                Disk::White => score.1 += 1,
                Disk::Empty => (),
            }
        }
        return score
    }
}

// Disk colours + empty for board
#[derive(Clone,Copy,Debug,PartialEq)]
enum Disk {
    Black,
    White,
    Empty,
}

// Player Struct
#[derive(Clone,Copy)]
pub struct Player {
    color: Disk,
    bot: bool,
}

impl Player {
    fn new(c: Disk, b: bool) -> Player {
        Player {
            color: c,
            bot: b,
        }
    }
    fn oppo(&self) -> Disk {
        match self.color {
            Disk::Black => Disk::White,
            Disk::White => Disk::Black,
            _ => panic!("Impossibrue color on oppo function"),
        }
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

    let player1 = Player::new(Disk::Black, false);
    let player2 = Player::new(Disk::White, true);

    // Game loop
    while ! finished {
        // Player turn (white)
        finished = turn(&mut board, player1);
        board.print();

        let bot_move = bot::bot_turn(&mut board, player2, depth);
        board.execute_move(&bot_move, player2);
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
//
//  return: bool; if true, the gameloop ends
fn turn(board: &mut Board, player: Player) -> bool {

    // Get valid moves
    let valid_moves = get_valid_moves(&board, player);
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
                board.execute_move(choice.1, player);
                break
            }
        }
    }
    return false
}

// Get valid moves: walk through the board and check what moves are valid
//  board: Board struct
//
//  return: A list of tuples;
//          tuple: Valid move with a list of flip-vectors
fn get_valid_moves(board: &Board, player: Player) -> Vec<Move> {
    let mut valid_moves = Vec::new();
    // Iterate over all squares in the field
    for i in 0..64 {
        // If the square isn't empty go to next
        if board.field[i] != Disk::Empty {
            continue
        }
        // Get all neighbouring squares; if no opponent piece is found go to next
        let neighbours = check_neighbours(board, i, player);
        if neighbours.len() == 0 {
            continue
        }
        // Check validity of move for neighbouring pieces
        let flips = get_flips(board, &neighbours, i, player);
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
//
//  return: Vector of neighboring opponents
pub fn check_neighbours(board: &Board, pos: usize, player: Player) -> Vec<usize> {
    let mut neighbours = Vec::new();
    // Check left of position
    // Check if we are not in left column
    if pos != 0 && pos % 8 != 0 {
        if board.field[pos - 1] == player.oppo() {
            neighbours.push(pos - 1);
        }
    }
    // Check right of position
    // Check if we are not in right colum
    if pos != 63 && (pos + 1) % 8 != 0 {
        if board.field[pos + 1] == player.oppo() {
            neighbours.push(pos + 1);
        }
    }
    // Check above position
    // Check if we are not on top row
    if pos > 7 {
        if board.field[pos - 8] == player.oppo() {
            neighbours.push(pos - 8);
        }
        if (pos + 1) % 8 != 0 && board.field[pos - 7] == player.oppo() {
            neighbours.push(pos - 7);
        }
        if pos % 8 != 0 && board.field[pos - 9] == player.oppo() {
            neighbours.push(pos - 9);
        }
    }
    // Check below position
    // Check if we're not on bottom row
    if pos < 54 {
        if board.field[pos + 8] == player.oppo() {
            neighbours.push(pos + 8);
        }
        if pos % 8 != 0&& board.field[pos + 7] == player.oppo() {
            neighbours.push(pos + 7);
        }
        if (pos + 2) % 8 != 0 && board.field[pos + 9] == player.oppo() {
            neighbours.push(pos + 9);
        }
    }
    return neighbours
}

// Get targets that will be flipped
//  board:    Board
//  targets:  Neighbouring opponent pieces
//  position: Position on board
//
//  return:   List of vectors; vectors are pieces of opponent that'll be flipped
pub fn get_flips(board: &Board, targets: &Vec<usize>, position: usize, player: Player) -> Vec<Vec<usize>> {
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
            if step == -1 && pos % 8 == 0 {
                break
            }

            // Get disk of next position
            let next = board.field[(pos + step) as usize];

            // If next pos is None there's no disk
            // If next pos matches own disk; add to list
            // If next pos matches opp disk; add to list and push list to flip-list
            if next == Disk::Empty {
                break
            } else if next == player.oppo() {
                fp.push(pos as usize);
            } else if next == player.color {
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
        println!("Ready...\nSteady....\nFIGHT!\n");
        let mut board = Board::start();
        let mut stall = 0;
        loop {
            let depth = 3;
            let bot0 = Player::new(Disk::Black, true);
            let bot1 = Player::new(Disk::White, true);

            let val_moves = get_valid_moves(&board, bot0);
            println!("MOVE *");
            if val_moves.len() == 0 {
                println!("STALLLING *");
                stall += 1;
                if stall >= 2 {
                    println!("BREAKING *");
                    break
                } else {
                    continue
                }
            } else {
                stall = 0;
            }
            let bot_move = bot::bot_turn(&mut board, bot0, depth);
            println!("BOT MOVE: {:?}", bot_move);
            board.execute_move(&bot_move, bot0);
            board.print();

            let val_moves = get_valid_moves(&board, bot1);
            println!("MOVE o");
            if val_moves.len() == 0 {
                println!("STALLING o");
                stall += 1;
                if stall >= 2 {
                    println!("BREAKING o");
                    break
                } else {
                    continue
                }
            } else {
                stall = 0;
            }
            let bot_move = bot::bot_turn(&mut board, bot1, depth);
            println!("BOT MOVE: {:?}", bot_move);
            board.execute_move(&bot_move, bot1);
            board.print();
        }
        let final_score = board.score();
        println!("Final score: {}-{}", final_score.0, final_score.1);
        if final_score.1 > final_score.0 {
            println!("o-Bot Wins!!!");
        } else {
            println!("*-Bot Wins!!!");
        }
    }
}
