//
// IMPORTS
// std
use std::io::{self};

// mods
//mod bot;

//
// STRUCTS
#[derive(Debug)]
struct Node {
    //        move   score
    content: (usize, isize),
    children: Vec<Node>,
}

impl Node {
    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
    fn new((mv, score): (usize, isize)) -> Node {
        Node {
            content: (mv, score),
            children: Vec::new(),
        }
    }
}

// Clone for cloning
pub struct Board {
    field: [usize; 64],
}

// Board implementations
impl Board {
    // Setup board
    fn start() -> Board {
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
    // Print board
    fn print(&self) {
        for i in 0..8 {
            let mut v = Vec::new();
            for j in 0..8 {
                v.push(&self.field[(i*8)+j]);
            }
            println!("{:?}", v);
        }
        println!("");
    }

    // Set a piece
    fn execute_move(&mut self, stones: &(usize, Vec<Vec<usize>>), opponent_color: usize) {
        let color = match opponent_color {
            1 => 2,
            2 => 1,
            _ => panic!("Impossibru!"),
        };
        self.field[stones.0] = color;
        for vector in &stones.1 {
            for x in vector {
                self.field[*x] = color;
            }
        }
    }

}

//
// MAIN
//
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

//      board              color = color of opponent
fn turn(mut board: &mut Board, color: usize) {
    // Get valid moves
    // Vec<usize, Vec<usize>>
    //     move   flips
    let valid_moves = get_valid_moves(&board, color);

    // Print valid moves
    println!("Valid moves:");
    for m in &valid_moves {
        println!("{}", m.0);
    }

    let mut valid = false;
    // Get input
    while ! valid {
        println!("Chose your move");
        let mut player_input = String::new();
        io::stdin().read_line(&mut player_input).expect("Could not read?");
        let player_input_trimmed = player_input.trim();

        // Cast to int
        let player_input_int = match player_input_trimmed.parse::<usize>() {
            Ok(n) => n,
            Err(e) => {
                println!("Error: {}", e);
                continue
            },
        };
        // If choice is in the list of moves, execute move and break
        for choice in &valid_moves {
            if choice.0 == player_input_int {
                valid = true;
                // EXECUTE
                board.execute_move(choice, color);
                break
            }
        }
    }
}

// Get valid moves: walk through the board and check what moves are valid
//                                                      Move , List of vectors that will flip
fn get_valid_moves(board: &Board, color: usize) -> Vec<(usize, Vec<Vec<usize>>)> {
    let mut valid_moves = Vec::new();
    for i in 0..63 {
        if board.field[i] != 0 {
            continue
        }
        let neighbours = check_neighbours(board, i, color);
        if neighbours.len() == 0 {
            continue
        }
        let flips = get_flips(board, &neighbours, i, color);
        if flips.len() == 0 {
            continue
        }
        valid_moves.push((i, flips));
    }
    return valid_moves
}

// Check for neighbours
pub fn check_neighbours(board: &Board, pmove: usize, color: usize) -> Vec<usize> {
    let mut targets = Vec::new();
    // Corner cases
    if pmove != 0 && pmove % 8 != 0 {
        if board.field[pmove - 1] == color {
            targets.push(pmove - 1);
        }
    }
    // Corner cases
    if pmove != 63 && (pmove + 1) % 8 != 0 {
        if board.field[pmove + 1] == color {
            targets.push(pmove + 1);
        }
    }
    // Edge cases; do not wrap around
    if pmove > 8 {
        if board.field[pmove - 8] == color {
            targets.push(pmove - 8);
        }
        if pmove % 8 != 0 {
            if board.field[pmove - 7] == color {
                targets.push(pmove - 7);
            }
        }
        if (pmove + 1) % 8 != 0 {
            if board.field[pmove - 9] == color {
                targets.push(pmove - 9);
            }
        }
    }
    // Edge cases; do not wrap around
    if pmove < 56 {
        if board.field[pmove + 8] == color {
            targets.push(pmove + 8);
        }
        if pmove % 8 != 0 {
            if board.field[pmove + 7] == color {
                targets.push(pmove + 7);
            }
        }
        if (pmove + 1) % 8 != 0 {
            if board.field[pmove + 9] == color {
                targets.push(pmove + 9);
            }
        }
    }
    return targets
}

// Get targets that will be flipped
pub fn get_flips(board: &Board, targets: &Vec<usize>, pm: usize, color: usize) -> Vec<Vec<usize>> {
    // Dirty
    let d = match color {
        1 => 2,
        2 => 1,
        _ => 0,
    };

    // Use isize; going negative :O
    let pi = pm as isize;
    let mut flips = Vec::new();

    // Iterate through all possible targets
    for t in targets {
        // Position
        let mut pos = pi;
        // Isize target
        let ti = *t as isize;
        // Step
        let step = ti - pi;
        // Flip positions
        let mut fp = Vec::new();
        // Loop until we find our own stone, or edge of board
        loop {
            // Walk
            pos = pos + step;
            // Break if we find edge or empty square
            if (pos + step) < 0 || (pos + step) > 63 || pos % 8 == 0 || (pos + 1) % 8 == 0 || board.field[(pos + step) as usize] == 0 {
                break
            // Add position if next pos is enemy
            } else if board.field[(pos + step) as usize] == color {
                fp.push(pos as usize);
            // Add position if next pos is us, and push all flip positions
            } else if board.field[(pos + step) as usize] == d {
                fp.push(pos as usize);
                flips.push(fp);
                break
            }
        }
    }
    return flips
}

