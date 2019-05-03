use crate::*;

use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Debug)]
struct Node {
    mvval: usize,
    score: isize,
    children: Vec<Node>,
}

impl Node {
    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

pub fn bot_turn(mut board: &mut Board, color: usize, depth: usize) -> usize {

    let valid_moves = get_valid_moves(&board, color);

    let mut handles = Vec::new();

    let mut top_node = Node {
        mvval: usize::max_value(),
        score: 0,
        children: Vec::new(),
    };

    for i in valid_moves {
        let mut mv_board = board.clone();
        mv_board.execute_move(&i, color);
        let handle = thread::spawn(move || {
            let t_node = Node {
                mvval: i.0,
                score: 0,
                children: Vec::new(),
            };
            let t_color = color;
            let result_node = bot_rec(&mv_board, t_color, 1, t_node);
            return result_node
        });
        handles.push(handle);
    }

    for handle in handles {
        let res_nod = handle.join().unwrap();
        top_node.add_child(res_nod);
    }
    let mut bot_move = 0;
    let mut bot_max_score = isize::min_value();
    for c in &top_node.children {
        if c.score > bot_max_score {
            bot_max_score = c.score;
            bot_move = c.mvval;
        }
    }

    return bot_move
}

fn bot_rec(mut board: &Board, color: usize, depth: usize, mut node: Node) -> Node {
    if depth > 8 {
        return node;
    }
    // Get valid moves
    let valid_moves = get_valid_moves(&board, color);
    if valid_moves.len() == 0 {
        return node;
    }

    // Iterate over valid moves, recursive
    for mv in &valid_moves {
        let mut board_new = board.clone();
        let color_new = match color {
            1 => 2,
            2 => 1,
            _ => panic!("Bot panic color swap."),
        };

        board_new.execute_move(&mv, color);

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
    for child in &node.children {
        node.score += child.score;
    }
    return node;
}
