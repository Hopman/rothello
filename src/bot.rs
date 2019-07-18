use crate::*;

use std::thread;

use rand;

// Simple node struct for tree-like moves
#[derive(Clone)]
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

// Public function bot's turn
//  board:      Board
//  max_depth:  maximum depth of Nodes
//
//  return:     Return move position
pub fn bot_turn(board: &mut Board, player: Player) -> Option<Move> {
    // Get valid moves for bot
    let valid_moves = get_valid_moves(&board, player);
    if valid_moves.len() == 0 {
        return None
    }

    // Keep original player
    let bot = player.clone();

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

            let new_player = Player::new(player.oppo(), true);

            let result_node = recursive_bot(&mv_board, bot, new_player, 0, thread_node);

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

        } else if child.score == bot_max_score {
            if rand::random() {
                bot_move = child.mv.clone();
                bot_max_score = child.score;
            }
        }
    }
    // Return 'best' bot move
    return Some(bot_move)
}

// Recursive bot function
//  board:      Board Struct
//  player:     Player Struct
//  max_depth:  Maximum depth of nodes
//  depth:      Depth of current node
//  node:       Parent Node
//
//  return:     Node
fn recursive_bot(board: &Board, bot: Player, player: Player, depth: usize, mut node: Node) -> Node {
    // Expect depth
    if depth >= *MAX_DEPTH {
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
        let player_new = Player::new(player.oppo(), true);

        // Execute move on the new board
        board_new.execute_move(&mv, player);

        // Initiate new node
        let new_node = Node {
            mv: mv.clone(),
            score: evaluate(&board_new, mv.mv_int, bot, player),
            children: Vec::new(),
        };

        // Recursive
        let child_node = recursive_bot(&mut board_new, bot, player_new, depth + 1, new_node);

        node.add_child(child_node);
    }

    // Increment scores
    for child in &node.children {
        node.score += (child.score / ((depth * *DEPTH_WEIGHT) as isize + 1));
    }
    return node
}

// Evaluate Node score
fn evaluate(board: &Board, mv: usize, bot: Player, player: Player) -> isize {
    // score: tuple (black, white)
    let score_tuple = board.score();

    let mut score_int = match player.color {
        Disk::Black => {
            score_tuple.0 as isize - score_tuple.1 as isize
        }
        Disk::White => {
            score_tuple.1 as isize - score_tuple.0 as isize
        }
        Disk::None => panic!("Player color cannot be Emtpy. [bot::calc_score]")
    };

    score_int += match mv {
        0 | 7 | 56 | 63 => *CORNER_VALUE,
        _ => 0,
    };

    if score_tuple.2 && score_int <= 0 {
        score_int -= *WINLOSE_VALUE;
    }
    if score_tuple.2 && score_int >= 0 {
        score_int += *WINLOSE_VALUE;
    }

    if player.color != bot.color {
        score_int *= -1;
    }
    return score_int
}
