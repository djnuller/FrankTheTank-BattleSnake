use rocket::serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

// API and Response Objects
// See https://docs.battlesnake.com/api

#[derive(Deserialize, Serialize, Debug)]
pub struct Game {
    pub id: String,
    pub ruleset: HashMap<String, Value>,
    pub timeout: i32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Board {
    pub height: i32,
    pub width: i32,
    pub food: Vec<Coord>,
    pub snakes: Vec<Battlesnake>,
    pub hazards: Vec<Coord>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Battlesnake {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub body: Vec<Coord>,
    pub head: Coord,
    pub length: i32,
    pub latency: String,
    pub shout: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Eq, Hash, Clone, Copy)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    // self is head
    pub fn successors(
        &self,
        hazards: &Vec<Coord>,
        battlesnakes: &Vec<Battlesnake>,
        body: &Vec<Coord>,
        _board: &Board,
    ) -> Vec<Coord> {
        let &Coord { x: _, y: _ } = self;
        let mut is_move_safe: HashMap<_, _> = vec![
            ("up", true),
            ("down", true),
            ("left", true),
            ("right", true),
        ]
        .into_iter()
        .collect();
        // ugly but for performance
        Coord::prevent_walls(self, &_board, &mut is_move_safe);
        Coord::prevent_other_snakes(self, battlesnakes, &mut is_move_safe);
        Coord::prevent_hazards(self, hazards, &mut is_move_safe);
        Coord::prevent_self_destruction(self, body, &mut is_move_safe);

        // Coord::get_succesors(&self, hazards, battlesnakes, body, _board)
        is_move_safe
            .into_iter()
            .filter(|&(_, v)| v)
            .map(|(k, _)| Coord::get_coord_from_string(self, k))
            .collect::<Vec<Coord>>()
    }

    pub fn neighbours(
        &self,
        hazards: &Vec<Coord>,
        battlesnakes: &Vec<Battlesnake>,
        body: &Vec<Coord>,
        _board: &Board,
    ) -> Vec<(Coord, usize)> {
        let &Coord { x: _, y: _ } = self;
        let mut is_move_safe: HashMap<_, _> = vec![
            ("up", true),
            ("down", true),
            ("left", true),
            ("right", true),
        ]
        .into_iter()
        .collect();
        // ugly but for performance
        Coord::prevent_walls(self, &_board, &mut is_move_safe);
        Coord::prevent_other_snakes(self, battlesnakes, &mut is_move_safe);
        Coord::prevent_hazards(self, hazards, &mut is_move_safe);
        Coord::prevent_self_destruction(self, body, &mut is_move_safe);

        let moves = is_move_safe
            .into_iter()
            .map(|(k, v)| {
                (
                    Coord::get_coord_from_string(self, k),
                    if v { 1 } else { 999 },
                )
            })
            .collect();
        moves
    }

    pub fn distance(&self, other: &Coord) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

    pub fn prevent_walls(&self, _board: &Board, is_move_safe: &mut HashMap<&str, bool>) {
        if self.x == 0 {
            is_move_safe.insert("left", false);
        }
        if self.x == _board.width - 1 {
            is_move_safe.insert("right", false);
        }
        if self.y == 0 {
            is_move_safe.insert("down", false);
        }
        if self.y == _board.height - 1 {
            is_move_safe.insert("up", false);
        }
    }

    fn get_coord_from_string(&self, _move: &str) -> Coord {
        let moves = vec![
            Coord {
                x: self.x + 1,
                y: self.y,
            }, // right 0
            Coord {
                x: self.x - 1,
                y: self.y,
            }, // left 1
            Coord {
                x: self.x,
                y: self.y + 1,
            }, // up 2
            Coord {
                x: self.x,
                y: self.y - 1,
            }, // down 3
        ];
        match _move {
            "right" => moves[0],
            "left" => moves[1],
            "up" => moves[2],
            "down" => moves[3],
            _ => moves[1], // no safe moves
        }
    }

    pub fn prevent_self_destruction(
        &self,
        _body: &Vec<Coord>,
        is_move_safe: &mut HashMap<&str, bool>,
    ) {
        if _body.contains(&Coord {
            x: self.x + 1,
            y: self.y,
        }) {
            is_move_safe.insert("right", false);
        }
        if _body.contains(&Coord {
            x: self.x - 1,
            y: self.y,
        }) {
            is_move_safe.insert("left", false);
        }
        if _body.contains(&Coord {
            x: self.x,
            y: self.y + 1,
        }) {
            is_move_safe.insert("up", false);
        }
        if _body.contains(&Coord {
            x: self.x,
            y: self.y - 1,
        }) {
            is_move_safe.insert("down", false);
        }
    }

    pub fn prevent_hazards(&self, hazards: &Vec<Coord>, is_move_safe: &mut HashMap<&str, bool>) {
        for (_i, hazards) in hazards.iter().enumerate() {
            if hazards.eq(&Coord {
                x: self.x + 1,
                y: self.y,
            }) {
                is_move_safe.insert("right", false);
            }
            if hazards.eq(&Coord {
                x: self.x - 1,
                y: self.y,
            }) {
                is_move_safe.insert("left", false);
            }
            if hazards.eq(&Coord {
                x: self.x,
                y: self.y + 1,
            }) {
                is_move_safe.insert("up", false);
            }
            if hazards.eq(&Coord {
                x: self.x,
                y: self.y - 1,
            }) {
                is_move_safe.insert("down", false);
            }
        }
    }

    pub fn prevent_other_snakes(
        &self,
        snakes: &Vec<Battlesnake>,
        is_move_safe: &mut HashMap<&str, bool>,
    ) {
        for (_i, snake) in snakes.iter().enumerate() {
            if snake.body.contains(&Coord {
                x: self.x + 1,
                y: self.y,
            }) {
                is_move_safe.insert("right", false);
            }
            if snake.body.contains(&Coord {
                x: self.x - 1,
                y: self.y,
            }) {
                is_move_safe.insert("left", false);
            }
            if snake.body.contains(&Coord {
                x: self.x,
                y: self.y + 1,
            }) {
                is_move_safe.insert("up", false);
            }
            if snake.body.contains(&Coord {
                x: self.x,
                y: self.y - 1,
            }) {
                is_move_safe.insert("down", false);
            }
        }
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GameState {
    pub game: Game,
    pub turn: i32,
    pub board: Board,
    pub you: Battlesnake,
}
