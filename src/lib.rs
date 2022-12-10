use log::info;
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

#[derive(Deserialize, Serialize, Debug)]
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

    pub fn get_succesors(
        &self,
        hazards: &Vec<Coord>,
        battlesnakes: &Vec<Battlesnake>,
        body: &Vec<Coord>,
        _board: &Board,
    ) -> Vec<Coord> {
        let mut successors = Vec::new();
        for dx in 0.._board.width {
            for dy in 0.._board.height {
                // Omit diagonal moves (and moving to the same position)
                if (dx + dy) != 1 {
                    continue;
                }
                let new_position = &Coord {
                    x: self.x + dx,
                    y: self.y + dy,
                };
                if new_position.x < 0
                    || new_position.x >= _board.width
                    || new_position.y < 0
                    || new_position.y >= _board.height
                {
                    continue;
                }
                let mut is_move_safe: HashMap<_, _> = vec![
                    ("up", true),
                    ("down", true),
                    ("left", true),
                    ("right", true),
                ]
                .into_iter()
                .collect();

                Coord::prevent_walls(new_position, &_board, &mut is_move_safe);
                Coord::prevent_other_snakes(new_position, battlesnakes, &mut is_move_safe);
                Coord::prevent_hazards(new_position, hazards, &mut is_move_safe);
                Coord::prevent_self_destruction(new_position, body, &mut is_move_safe);

                if is_move_safe
                    .iter()
                    .filter(|&(_, v)| *v)
                    .map(|(k, _)| k)
                    .collect::<Vec<_>>()
                    .is_empty()
                {
                    info!("Empty go further!");
                    continue;
                }

                let safe_coords: Vec<Coord> = is_move_safe
                    .into_iter()
                    .filter(|&(_, v)| v)
                    .map(|(k, _)| Coord::get_coord_from_string(self, k))
                    .collect::<Vec<Coord>>();
                if safe_coords.len() > 0 {
                    successors.push(Coord {
                        x: new_position.x,
                        y: new_position.y,
                    });
                }
            }
        }
        successors
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
