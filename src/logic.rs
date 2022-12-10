use log::info;
use pathfinding::prelude::astar;
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

fn find_nearest_food_astar(
    head: &Coord,
    food: &Vec<Coord>,
    hazards: &Vec<Coord>,
    battlesnakes: &Vec<Battlesnake>,
    body: &Vec<Coord>,
    _board: &Board,
) -> Option<Vec<Coord>> {
    let mut best_path: Vec<Vec<Coord>> = Vec::new();
    if food.is_empty() {
        let _result = astar(
            head,
            |p| p.neighbours(hazards, battlesnakes, body, _board),
            |p| p.distance(&body[body.len() - 1]) / 3,
            |p| *p == body[body.len() - 1],
        );

        if _result.is_some() {
            let (result, _) = _result.unwrap();
            best_path.push(result);
        }
    }
    for (_i, _food) in food.iter().enumerate() {
        let _result = astar(
            head,
            |p| p.neighbours(hazards, battlesnakes, body, _board),
            |p| p.distance(&body[body.len() - 1]) / 3,
            |p| *p == *_food,
        );

        if _result.is_some() {
            let (result, _) = _result.unwrap();
            best_path.push(result);
        }
    }
    if best_path.is_empty() {
        return None;
    }
    best_path.sort_by(|a, b| a.len().cmp(&b.len()));
    return Some(best_path[0].clone());
}

fn find_nearest_food_bfs(
    head: &Coord,
    food: &Vec<Coord>,
    hazards: &Vec<Coord>,
    battlesnakes: &Vec<Battlesnake>,
    body: &Vec<Coord>,
    _board: &Board,
) -> Option<Vec<Coord>> {
    let mut best_path: Vec<Vec<Coord>> = Vec::new();
    if food.is_empty() {
        let _result = bfs(
            head,
            |p| p.successors(hazards, battlesnakes, body, _board),
            |p| *p == body[body.len()], // hunt tail
        );
        if _result.is_some() {
            let result = _result.unwrap();
            best_path.push(result);
        }
    }
    for (_i, _food) in food.iter().enumerate() {
        let _result = bfs(
            head,
            |p| p.successors(hazards, battlesnakes, body, _board),
            |p| *p == _food.clone(),
        );
        if _result.is_some() {
            let result = _result.unwrap();
            best_path.push(result);
        }
    }
    if best_path.is_empty() {
        return None;
    }
    best_path.sort_by(|a, b| a.len().cmp(&b.len()));
    return Some(best_path[0].clone());
}

fn suggested_best_move(
    head: &Coord,
    food: &Vec<Coord>,
    hazards: &Vec<Coord>,
    battlesnakes: &Vec<Battlesnake>,
    body: &Vec<Coord>,
    _board: &Board,
) -> Option<Vec<&'static str>> {
    let nearest_food = find_nearest_food_astar(head, food, hazards, battlesnakes, body, _board);
    if nearest_food.is_some() {
        let path = nearest_food.unwrap();
        info!("Path contains {:?}", path);
        // take next step
        let mut made_path = Vec::new();
        for (_i, _path) in path.iter().enumerate() {
            if head.x > path[_i].x {
                made_path.push("left");
            } else if head.x < path[_i].x {
                made_path.push("right");
            }
            if head.y > path[_i].y {
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

fn log_moves(head: &Coord, method: &str, is_move_safe: &mut HashMap<&str, bool>) {
    info!(
        "head: {:?} is_move_safe after {} up ({:?}), down ({:?}), left ({:?}), right ({:?})",
        head,
        method,
        is_move_safe.get("up"),
        is_move_safe.get("down"),
        is_move_safe.get("left"),
        is_move_safe.get("right")
    );
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
    // remove tail
    let body = &mut you.body.clone();
    body.pop();
    let head = &body[0]; // Coordinates of your head
    let neck = &body[1]; // Coordinates of your "neck"

    prevent_backwards(head, neck, &mut is_move_safe);

    let suggested_best_move = suggested_best_move(
        head,
        &_board.food,
        &_board.hazards,
        &_board.snakes,
        &body,
        _board,
    );
    info!("suggested_best_move : {:?}", suggested_best_move);
    let _snakes = &_board
        .snakes
        .clone()
        .into_iter()
        .filter(|s| !s.id.eq(&you.id) && !s.name.eq(&you.name))
        .collect::<Vec<Battlesnake>>();

    info!("snakes! {:?}", _snakes);

    head.prevent_hazards(&_board.hazards, &mut is_move_safe);
    log_moves(head, "prevent_hazards", &mut is_move_safe);
    head.prevent_other_snakes(_snakes, &mut is_move_safe);
    log_moves(head, "prevent_other_snakes", &mut is_move_safe);
    head.prevent_self_destruction(&body, &mut is_move_safe);
    log_moves(head, "prevent_self_destruction", &mut is_move_safe);
    head.prevent_walls(_board, &mut is_move_safe);
    log_moves(head, "prevent_walls", &mut is_move_safe);
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
    }
    if safe_moves.len() > 0 && !safe_moves.contains(&chosen) {
        chosen = safe_moves.choose(&mut rand::thread_rng()).unwrap();
    }
    let end = std::time::Instant::now();
    let duration = end - start;
    info!("Logic took {}ns", duration.as_nanos());
    info!("MOVE {}: {}", turn, chosen);
    return json!({ "move": chosen });
}
