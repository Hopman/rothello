use crate::*;

use std::thread;

// Simple node struct for tree-like moves
#[derive(Clone, Debug)]
struct Node {
    mvval: usize,
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
//  board: Board
//  color: opponent's color
//
//  return: Return move position
pub fn bot_turn(board: &mut Board, color: usize) -> usize {

    // Get valid moves for bot
    let valid_moves = get_valid_moves(&board, color);

    // Create top node
    let mut top_node = Node {
        mvval: usize::max_value(),
        score: 0,
        children: Vec::new(),
    };

    // Handles for threading
    let mut handles = Vec::new();

    // For all possible moves, spawn a thread
    for i in valid_moves {

        // Execute the possible move first on cloned board
        let mut mv_board = board.clone();
        mv_board.execute_move(&i, color);

        // Spawn threads
        let handle = thread::spawn(move || {
            // Thread node
            let t_node = Node {
                mvval: i.0,
                score: 0,
                children: Vec::new(),
            };
            // Necessary?
            let t_color = color;
            let result_node = bot_rec(&mv_board, t_color, 0, t_node);

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
    let mut bot_move = 0;
    let mut bot_max_score = isize::min_value();
    for c in &top_node.children {
        if c.score > bot_max_score {
            bot_max_score = c.score;
            bot_move = c.mvval;
        }
    }

    // Return 'best' bot move
    return bot_move
}

// Recursive bot function
//  board: Board
//  color: Opponent's color
//  depth: Depth of recursion
//  node:  'parent' Node
//
//  return: Child Node
fn bot_rec(board: &Board, color: usize, depth: usize, mut node: Node) -> Node {
    // Expect depth
    if depth > 8 {
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
            mvval: mv.0,
            score: {
                let bscore = board.score();
                bscore.1 as isize - bscore.0 as isize
            },
            children: Vec::new(),
        };

        // Recursive
        let child_node = bot_rec(&mut board_new, color_new, depth + 1, new_node);
        node.add_child(child_node);
    }

    // Increment scores
    for child in &node.children {
        node.score += child.score;
    }
    return node
}
