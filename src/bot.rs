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
//  max_depth:  maximum depth of Nodes
//
//  return:     Return move position
pub fn bot_turn(board: &mut Board, player: Player, max_depth: usize) -> Move {
    // Get valid moves for bot
    let valid_moves = get_valid_moves(&board, player);

    // Create top node
    let mut top_node = Node {
        mv: Move::new(),
        score: 0,
        children: Vec::new(),
    };

    // Handles for threading
    let mut handles = Vec::new();

    // For all possible moves, spawn a thread
    for valid_move in valid_moves {

        // Execute the possible move first on cloned board
        let mut mv_board = board.clone();
        mv_board.execute_move(&valid_move, player);

        // Spawn threads
        let handle = thread::spawn(move || {
            // Thread node
            let thread_node = Node {
                mv: valid_move,
                score: 0,
                children: Vec::new(),
            };

            let new_player = Player::new(player.oppo(), !player.bot);

            let result_node = bot_rec(&mv_board, new_player, max_depth, 0, thread_node);

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
    println!("MAX SCORE>>>: {:#?}", bot_max_score);
    // Return 'best' bot move
    return bot_move
}

// Recursive bot function
//  board:      Board
//  max_depth:  Maximum depth of nodes
//  depth:      Depth of recursion
//  node:       'parent' Node
//
//  return:     Child Node
fn bot_rec(board: &Board, player: Player, max_depth: usize, depth: usize, mut node: Node) -> Node {
    // Expect depth
    if depth > max_depth {
        return node;
    }

    // Get valid moves
    let valid_moves = get_valid_moves(&board, player);

    // If there's no moves, collapse
    if valid_moves.len() == 0 {
        return node;
    }

    // Iterate over valid moves, recursive
    for mv in &valid_moves {
        // Clone board for child
        let mut board_new = board.clone();

        // Flip color
        let player_new = Player::new(player.oppo(), !player.bot);
        // Execute move on the new board
        board_new.execute_move(&mv, player);

        // Initiate new node
        let new_node = Node {
            mv: mv.clone(),
            score: calc_score(&board_new, mv.mv_int, player, depth + 1),
            children: Vec::new(),
        };

        // Recursive
        let child_node = bot_rec(&mut board_new, player_new, max_depth, depth + 1, new_node);

        node.add_child(child_node);
    }

    // Increment scores
    for child in &node.children {
        node.score += child.score;
    }
    return node
}

// Calculate score for a Node
fn calc_score(board: &Board, mv: usize, player: Player, depth: usize) -> isize {
    // score: tuple (black, white)
    let score = board.score();

    // Basic board score; Return score is (my piece count - their piece count)
    let mut return_score = match player.color {
        Disk::Black => {
            if score.0 == 0 {
                -1_000
            } else if score.1 == 0 {
                1_000
            } else {
                score.1 as isize - score.0 as isize
            }
        },
        Disk::White => {
            if score.1 == 0 {
                -1_000
            } else if score.0 == 0 {
                1_000
            } else {
                score.0 as isize - score.1 as isize
            }
        }
        _ => panic!("Impossibure disk type in return score."),
    };

    match mv {
        0 | 7 | 56 | 63 => {
            return_score = 9_999_999_999;
        },
        _ => (),
    }

    // Retrun score; positive for me, negative for opponent
    if player.bot {
        return (return_score / (depth * depth * depth * depth) as isize)
    } else {
        return (return_score / (depth * depth * depth * depth) as isize * -1)
    }
}
