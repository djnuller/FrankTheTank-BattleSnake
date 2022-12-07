use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{Battlesnake, Board, Game, Coord};

pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "", // TODO: Your Battlesnake Username
        "color": "#888888", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    });
}

pub fn start(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

pub fn end(_game: &Game, _turn: &u32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

fn prevent_backwards(head: &Coord, neck: &Coord, move_list: HashMap<_, _>) {
    if neck.x < head.x { // Neck is left of head, don't move left
        move_list.insert("left", false);

    } else if neck.x > head.x { // Neck is right of head, don't move right
        move_list.insert("right", false);

    } else if neck.y < head.y { // Neck is below head, don't move down
        move_list.insert("down", false);
    
    } else if neck.y > head.y { // Neck is above head, don't move up
        move_list.insert("up", false);
    }
}

// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &u32, _board: &Board, you: &Battlesnake) -> Value {
    
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    let my_head = &you.body[0]; // Coordinates of your head
    let my_neck = &you.body[1]; // Coordinates of your "neck"
    
    prevent_backwards(my_head, my_neck, is_move_safe);

    // TODO: Step 1 - Prevent your Battlesnake from moving out of bounds
    let board_width = &_board.width;
    let board_height = &_board.height;



    // TODO: Step 2 - Prevent your Battlesnake from colliding with itself
    // let my_body = &you.body;

    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    // let opponents = &board.snakes;

    // Are there any safe moves left?
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();
    
    // Choose a random move from the safe ones
    let chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap();

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;

    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}
