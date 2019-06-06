use crate::*;


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
//  board:      Board
//  color:      opponent's color
//  max_depth:  maximum depth of Nodes
//
//  return:     Return move position
pub fn bot_turn(board: &mut Board, color: usize, max_depth: usize) -> usize {

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
                mvval: i.mv_int,
                score: 0,
                children: Vec::new(),
            };
            // Necessary?
            let t_color = color;
            let result_node = bot_rec(&mv_board, t_color, max_depth, 0, t_node);

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
        if c.mvval == 0 {
            return c.mvval
        } else if c.mvval == 7 {
            return c.mvval
        } else if c.mvval == 56 {
            return c.mvval
        } else if c.mvval == 63 {
            return c.mvval
        }
        if c.score > bot_max_score {
            bot_max_score = c.score;
            bot_move = c.mvval;
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
fn bot_rec(board: &Board, color: usize, max_depth: usize, depth: usize, mut node: Node) -> Node {
    // Expect depth
    if depth > max_depth {
        return node;
    }

    // Get valid moves
    let valid_moves = get_valid_moves(&board, color);

    // If there's no moves, collapse
    if valid_moves.len() == 0 { return node;
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
            mvval: mv.mv_int,
            score: calc_score(&board_new, mv.mv_int, color),
            children: Vec::new(),
        };

        // Recursive
        let child_node = bot_rec(&mut board_new, color_new, max_depth, depth + 1, new_node);
        node.add_child(child_node);
    }

    // Increment scores
    for child in &node.children {
        node.score += child.score;
    }
    return node
}

fn calc_score(board: &Board, mv: usize, color: usize) -> isize {
    if color == 2 {
        let score = match mv {
            0 => -2500,
            7 => -2500,
            63 => -2500,
            56 => -2500,
            _ => {
                let board_score = board.score();
                board_score.1 as isize - board_score.0 as isize
            },
        };
        return score
    } else {
        let score = match mv {
            0 => 2500,
            7 => 2500,
            63 => 2500,
            56 => 2500,
            _ => {
                let board_score = board.score();
                board_score.1 as isize - board_score.0 as isize
            },
        };
        return score
    }
}
