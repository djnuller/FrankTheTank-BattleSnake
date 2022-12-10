use log::info;
use pathfinding::prelude::bfs;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;

use starter_snake_rust::{Battlesnake, Board, Coord, Game};

pub fn info() -> Value {
    info!("INFO");

    return json!({
        "apiversion": "1",
        "author": "Djnuller",
        "color": "#FFFF33",
        "head": "whale",
        "tail": "dragon",
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
    if _body.contains(&Coord {
        x: head.x + 1,
        y: head.y,
    }) {
        is_move_safe.insert("right", false);
    }
    if _body.contains(&Coord {
        x: head.x - 1,
        y: head.y,
    }) {
        is_move_safe.insert("left", false);
    }
    if _body.contains(&Coord {
        x: head.x,
        y: head.y + 1,
    }) {
        is_move_safe.insert("up", false);
    }
    if _body.contains(&Coord {
        x: head.x,
        y: head.y - 1,
    }) {
        is_move_safe.insert("down", false);
    }
}

fn find_nearest_food(
    head: &Coord,
    food: &Vec<Coord>,
    hazards: &Vec<Coord>,
    battlesnakes: &Vec<Battlesnake>,
    body: &Vec<Coord>,
    _board: &Board,
) -> Option<Vec<Coord>> {
    let mut best_path: Option<Vec<Coord>> = None;
    let mut shortest_distance: usize = 999;
    if food.is_empty() {
        let _result = bfs(
            head,
            |p| p.successors(hazards, battlesnakes, body, _board),
            |p| *p == body[body.len()], // hunt tail
        );
        info!("result {:?}", _result);
        if _result.is_some() {
            let result = _result.unwrap();

            if result.len() < shortest_distance {
                shortest_distance = result.len();
                best_path = Some(result);
            }
        }
    }
    for (_i, _food) in food.iter().enumerate() {
        let _result = bfs(
            head,
            |p| p.successors(hazards, battlesnakes, body, _board),
            |p| *p == _food.clone(),
        );
        info!("result with food {:?}", _result);
        if _result.is_some() {
            let result = _result.unwrap();
            if result.len() == shortest_distance {
                continue;
            }

            if result.len() < shortest_distance {
                shortest_distance = result.len();
                best_path = Some(result);
            }
        }
    }
    return best_path;
}

fn suggested_best_move(
    head: &Coord,
    food: &Vec<Coord>,
    hazards: &Vec<Coord>,
    battlesnakes: &Vec<Battlesnake>,
    body: &Vec<Coord>,
    _board: &Board,
) -> Option<Vec<&'static str>> {
    let nearest_food = find_nearest_food(head, food, hazards, battlesnakes, body, _board);
    info!("nearest_food: {:?}", nearest_food);
    if nearest_food.is_some() {
        let path = nearest_food.unwrap();
        // take next step
        let mut made_path = Vec::new();
        for (_i, _path) in path.iter().enumerate() {
            if head.x > path[_i].x {
                made_path.push("left");
            } else if head.x < path[_i].x {
                made_path.push("right");
            } else if head.y > path[_i].y {
                made_path.push("down");
            } else if head.y < path[_i].y {
                made_path.push("up");
            }
        }
        if made_path.len() > 0 {
            return Some(made_path);
        }
    }

    return None;
}

fn log_moves(method: &str, is_move_safe: &mut HashMap<&str, bool>) {
    info!(
        "is_move_safe after {} up ({}), down ({}), left ({}), right ({})",
        method,
        bool_to_str(is_move_safe.get("up")),
        bool_to_str(is_move_safe.get("down")),
        bool_to_str(is_move_safe.get("left")),
        bool_to_str(is_move_safe.get("right"))
    );
}

fn bool_to_str(optional_boolean: Option<&bool>) -> &'static str {
    if optional_boolean.unwrap().clone() {
        return "true";
    }
    return "false";
}

// See https://docs.battlesnake.com/api/example-move for available data
pub fn get_move(_game: &Game, turn: &i32, _board: &Board, you: &Battlesnake) -> Value {
    let start = std::time::Instant::now();
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
    log_moves("prevent_backwards", &mut is_move_safe);
    let suggested_best_move = suggested_best_move(
        head,
        &_board.food,
        &_board.hazards,
        &_board.snakes,
        &you.body,
        _board,
    );
    info!("suggested_best_move : {:?}", suggested_best_move);
    // TODO: Step 3 - Prevent your Battlesnake from colliding with other Battlesnakes
    // let opponents = &board.snakes;

    // Are there any safe moves left?
    let safe_moves = is_move_safe
        .into_iter()
        .filter(|&(_, v)| v)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    let mut chosen = "down";
    // default down
    if suggested_best_move.is_some() {
        let suggested_moves = suggested_best_move.unwrap();
        for moves in suggested_moves {
            if safe_moves.contains(&moves) {
                chosen = moves;
                break;
            }
        }
    } else if safe_moves.len() > 0 {
        chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap();
    }
    // Choose a random move from the safe ones

    // TODO: Step 4 - Move towards food instead of random, to regain health and survive longer
    // let food = &board.food;
    let end = std::time::Instant::now();
    let duration = end - start;
    info!("Logic took {}ns", duration.as_nanos());
    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}
