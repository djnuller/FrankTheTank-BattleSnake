use log::info;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

use crate::{Battlesnake, Board, Coord, Game};

pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "Djnuller", // TODO: Your Battlesnake Username
        "color": "#FFFF33", // TODO: Choose color
        "head": "default", // TODO: Choose head
        "tail": "default", // TODO: Choose tail
    });
}

pub fn start(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME START");
}

pub fn end(_game: &Game, _turn: &i32, _board: &Board, _you: &Battlesnake) {
    info!("GAME OVER");
}

fn prevent_backwards(head: &Coord, neck: &Coord, is_move_safe: &mut HashMap<&str, bool>) {
    if neck.x < head.x {
        // Neck is left of head, don't move left
        is_move_safe.insert("left", false);
    } else if neck.x > head.x {
        // Neck is right of head, don't move right
        is_move_safe.insert("right", false);
    } else if neck.y < head.y {
        // Neck is below head, don't move down
        is_move_safe.insert("down", false);
    } else if neck.y > head.y {
        // Neck is above head, don't move up
        is_move_safe.insert("up", false);
    }
}

fn prevent_walls(head: &Coord, _board: &Board, is_move_safe: &mut HashMap<&str, bool>) {
    // check if head is close to bottom or top, left or right
    if head.x == 0 {
        is_move_safe.insert("left", false);
    } else if head.x == _board.width - 1 {
        is_move_safe.insert("right", false);
    }
    if head.y == 0 {
        is_move_safe.insert("down", false);
    } else if head.y == _board.height - 1 {
        is_move_safe.insert("up", false);
    }
}

fn prevent_self_destruction(
    head: &Coord,
    _body: &Vec<Coord>,
    is_move_safe: &mut HashMap<&str, bool>,
) {
    // check if body is to the left or right to the head
    let _skip = 0;
    for (i, body_coord) in _body.iter().enumerate() {
        if i == _skip {
            continue;
        }
        info!(
            "body_coord x: {}, y: {}. head_coord x: {}, y: {}",
            body_coord.x, body_coord.y, head.x, head.y
        );

        if head.x - 1 == body_coord.y && head.y - 1 == body_coord.x {
            info!("cant move left");
            is_move_safe.insert("left", false);
        } else if head.x + 1 == body_coord.y && head.y + 1 == body_coord.x {
            info!("cant move right");
            is_move_safe.insert("right", false);
        } else if head.y - 1 == body_coord.x && head.x - 1 == body_coord.y {
            info!("cant move down");
            is_move_safe.insert("down", false);
        } else if head.y + 1 == body_coord.x && head.x + 1 == body_coord.y {
            info!("cant move up");
            is_move_safe.insert("up", false);
        }
    }
}

fn bool_to_str(optional_boolean: Option<&bool>) -> &'static str {
    if optional_boolean.unwrap().clone() {
        return "true";
    }
    return "false";
}

// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &i32, _board: &Board, you: &Battlesnake) -> Value {
    let mut is_move_safe: HashMap<_, _> = vec![
        ("up", true),
        ("down", true),
        ("left", true),
        ("right", true),
    ]
    .into_iter()
    .collect();

    let head = &you.body[0]; // Coordinates of your head
    let neck = &you.body[1]; // Coordinates of your "neck"

    prevent_backwards(head, neck, &mut is_move_safe);
    info!(
        "is_move_safe after prevent_backwards up ({}), down ({}), left ({}), right ({})",
        bool_to_str(is_move_safe.get("up")),
        bool_to_str(is_move_safe.get("down")),
        bool_to_str(is_move_safe.get("left")),
        bool_to_str(is_move_safe.get("right"))
    );
    prevent_self_destruction(head, &you.body, &mut is_move_safe);
    info!(
        "is_move_safe after prevent_self_destruction up ({}), down ({}), left ({}), right ({})",
        bool_to_str(is_move_safe.get("up")),
        bool_to_str(is_move_safe.get("down")),
        bool_to_str(is_move_safe.get("left")),
        bool_to_str(is_move_safe.get("right"))
    );
    prevent_walls(head, _board, &mut is_move_safe);
    info!(
        "is_move_safe after prevent_walls up ({}), down ({}), left ({}), right ({})",
        bool_to_str(is_move_safe.get("up")),
        bool_to_str(is_move_safe.get("down")),
        bool_to_str(is_move_safe.get("left")),
        bool_to_str(is_move_safe.get("right"))
    );
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
