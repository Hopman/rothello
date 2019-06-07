use crate::*;

use std::thread;

// Simple node struct for tree-like moves
#[derive(Clone, Debug)]
struct Node {
    mv: Move,
    score: isize,
    children: Vec<Node>,
}

// Add a child to the node
impl Node {
    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

// Public funciont
//  board:      Board
//  color:      opponent's color
//  max_depth:  maximum depth of Nodes
//
//  return:     Return move position
pub fn bot_turn(board: &mut Board, color: usize, max_depth: usize) -> Move {
    let wcai = match color {
        1 => 2,
        2 => 1,
        _ => panic!("Impossible what color am I."),
    };

    // Get valid moves for bot
    let valid_moves = get_valid_moves(&board, color);

    // Create top node
    let mut top_node = Node {
        mv: Move::new(),
        score: 0,
        children: Vec::new(),
    };

    // Handles for threading
    let mut handles = Vec::new();

    // For all possible moves, spawn a thread
    for v_move in valid_moves {

        // Execute the possible move first on cloned board
        let mut mv_board = board.clone();
        mv_board.execute_move(&v_move, color);

        // Spawn threads
        let handle = thread::spawn(move || {
            // Thread node
            let t_node = Node {
                mv: v_move,
                score: 0,
                children: Vec::new(),
            };
            // Necessary?
            let t_color = color;
            let result_node = bot_rec(&mv_board, wcai, t_color, max_depth, 0, t_node);

            // Return node
            return result_node
        });
        // Push handle to handles vector
        handles.push(handle);
    }

    // Walk trough the handle results
    for handle in handles {
        let res_nod = handle.join().unwrap();
        // Add threaded result node to top node
        top_node.add_child(res_nod);
    }

    // Get bot maximum score/move
    let mut bot_move = Move::new();
    let mut bot_max_score = isize::min_value();

    // TODO Fancy me
    for child in &top_node.children {
        if child.score > bot_max_score {
            bot_move = child.mv.clone();
            bot_max_score = child.score;
        }
    }

    // Return 'best' bot move
    return bot_move
}

// Recursive bot function
//  board:      Board
//  color:      Opponent's color
//  max_depth:  Maximum depth of nodes
//  depth:      Depth of recursion
//  node:       'parent' Node
//
//  return:     Child Node
fn bot_rec(board: &Board, wcai: usize, color: usize, max_depth: usize, depth: usize, mut node: Node) -> Node {
    // Expect depth
    if depth > max_depth {
        return node;
    }

    // Get valid moves
    let valid_moves = get_valid_moves(&board, color);

    // If there's no moves, collapse
    if valid_moves.len() == 0 {
        return node;
    }

    // Iterate over valid moves, recursive
    for mv in &valid_moves {
        // Clone board for child
        let mut board_new = board.clone();

        // Flip color
        let color_new = match color {
            1 => 2,
            2 => 1,
            _ => panic!("Bot panic color swap."),
        };

        // Execute move on the new board
        board_new.execute_move(&mv, color);

        // Initiate new node
        let new_node = Node {
            mv: mv.clone(),
            score: calc_score(&board_new, wcai, mv.mv_int, color, depth),
            children: Vec::new(),
        };

        // Recursive
        let child_node = bot_rec(&mut board_new, wcai, color_new, max_depth, depth + 1, new_node);
        node.add_child(child_node);
    }

    // Increment scores
    for child in &node.children {
        node.score += child.score;
    }
    return node
}

// Calculate score for a Node
fn calc_score(board: &Board, wcai: usize, mv: usize,  color: usize, depth: usize) -> isize {
    // score: tuple (black, white)
    let score = board.score();


    // Basic board score; Return score is (my piece count - their piece count)
    let mut return_score = match wcai {
        1 => {
            if score.0 == 0 {
                -1_000_000
            } else {
                score.1 as isize - score.0 as isize
            }
        },
        2 => {
            if score.1 == 0 {
                -1_000_000
            } else {
                score.0 as isize - score.1 as isize
            }
        }
        _ => panic!("Impossibru wcai match in calc_score:\nScore: {:?}\nWCAI: {:?}", score, wcai),
    };

    // If my color; postive, otherwhise negative
    match mv {
        0 | 7 | 56 | 63 => return_score = -20_000,
        _ => (),
    }

    // Retrun score; positive for me, negative for opponent
    if wcai == color {
        return return_score
    } else {
        return (return_score * -1)
    }
}
