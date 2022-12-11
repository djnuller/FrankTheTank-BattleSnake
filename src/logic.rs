use log::{debug, info};
use pathfinding::prelude::astar;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

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
    body: &Vec<Coord>,
    _board: &Board,
    hunt_tail: bool,
    you: &Vec<Coord>,
) -> Option<Vec<Coord>> {
    let mut best_path: Vec<Vec<Coord>> = Vec::new();
    if food.is_empty() || hunt_tail {
        let _result = find_path(_board, head, &body[body.len() - 1], you);
        if _result.is_some() {
            let (result, _) = _result.unwrap();
            best_path.push(result);
        }
    } else {
        for (_i, _food) in food.iter().enumerate() {
            let _result = find_path(_board, head, _food, you);

            if _result.is_some() {
                let (result, _) = _result.unwrap();
                best_path.push(result);
            }
        }
    }
    debug!("path {:?} is hunt_tail {}", best_path, hunt_tail);

    if best_path.is_empty() {
        return None;
    }
    best_path.sort_by(|a, b| a.len().cmp(&b.len()));
    return Some(best_path[0].clone());
}

fn suggested_best_move(
    head: &Coord,
    food: &Vec<Coord>,
    body: &Vec<Coord>,
    _board: &Board,
    hunt_tail: bool,
    you: &Vec<Coord>,
) -> Option<Vec<&'static str>> {
    let nearest_food = find_nearest_food_astar(head, food, body, _board, hunt_tail, you);
    if nearest_food.is_some() {
        let path = nearest_food.unwrap();
        debug!("Path contains {:?} head is at {:?}", path, head);
        // take next step
        let mut made_path = Vec::new();
        for (_i, _path) in path.iter().enumerate() {
            if head.x == _path.x {
                if head.y + 1 == _path.y {
                    made_path.push("up");
                } else if head.y - 1 == _path.y {
                    made_path.push("down");
                }
            }
            if head.y == _path.y {
                if head.x + 1 == _path.x {
                    made_path.push("right");
                } else if head.x - 1 == _path.x {
                    made_path.push("left");
                }
            }
        }
        if made_path.len() > 0 {
            return Some(made_path);
        }
    }

    return None;
}

fn log_moves(head: &Coord, method: &str, is_move_safe: &mut HashMap<&str, bool>) {
    debug!(
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
    let health = &you.health;

    let _snakes = &_board
        .snakes
        .clone()
        .into_iter()
        .filter(|s| !s.id.eq(&you.id) && !s.name.eq(&you.name))
        .collect::<Vec<Battlesnake>>();

    prevent_backwards(head, neck, &mut is_move_safe);
    // head.prevent_hazards(&_board.hazards, &mut is_move_safe);
    log_moves(head, "prevent_hazards", &mut is_move_safe);
    // head.prevent_other_snakes(_snakes, &mut is_move_safe);
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

    let suggested_best_move =
        suggested_best_move(head, &_board.food, &body, _board, false, &you.body);
    // health > &60 && &you.body.len() > &6
    debug!("suggested_best_move : {:?}", suggested_best_move);

    let mut chosen = "down";
    // default down
    if suggested_best_move.is_some() {
        let suggested_moves = suggested_best_move.unwrap();
        for moves in suggested_moves {
            if safe_moves.contains(&moves) {
                debug!("Chose the best move from astar {}", moves);
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

fn find_path(
    board: &Board,
    start: &Coord,
    goal: &Coord,
    you: &Vec<Coord>,
) -> Option<(Vec<Coord>, usize)> {
    let obstacles = board
        .hazards
        .iter()
        .chain(board.snakes.iter().flat_map(|s| s.body.iter()))
        .chain(you)
        .collect::<HashSet<_>>();

    let cost = |c1: &Coord, c2: &Coord| {
        let dx = c1.x.abs_diff(c2.x);
        let dy = c1.y.abs_diff(c2.x);
        (dx + dy) as usize
    };

    let neighbors = |c: &Coord| -> Vec<(Coord, usize)> {
        let mut neighbors: Vec<Coord> = Vec::new();
        if c.x > 0 {
            neighbors.push(Coord { x: c.x - 1, y: c.y });
        }
        if c.x < board.width - 1 {
            neighbors.push(Coord { x: c.x + 1, y: c.y });
        }
        if c.y > 0 {
            neighbors.push(Coord { x: c.x, y: c.y - 1 });
        }
        if c.y < board.height - 1 {
            neighbors.push(Coord { x: c.x, y: c.y + 1 });
        }
        // Filter the list of neighbors to only include coordinates
        // that are not in the set of obstacles
        neighbors
            .into_iter()
            .filter(|c| !obstacles.contains(c))
            .map(|p| (p, 1 as usize))
            .collect()
    };

    astar(
        &start,
        |c| neighbors(c),
        |c| cost(c, goal), // Use the cost function defined earlier to calculate the cost of moving from c to goal
        |c| c == goal, // The search is successful when the current coordinate is the goal coordinate
    )
}
