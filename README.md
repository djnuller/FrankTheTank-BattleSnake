```
use pathfinding::prelude::astar;
use std::collections::HashMap;
use std::hash::Hash;

fn pathfinding(snake: VecDeque<(usize, usize)>, board: [[i32; 10]; 10]) -> usize {
    // Create a closure for the A* heuristic function
    let heuristic = |a: &(usize, usize), b: &(usize, usize)| -> u64 {
        (9 - a.0).abs() as u64 + (9 - a.1).abs() as u64
    };

    // Create a closure for the A* neighbor function
    let neighbor = |&(x, y): &(usize, usize)| -> Vec<((usize, usize), u64)> {
        let mut neighbors = Vec::new();
        for i in -1..=1 {
            for j in -1..=1 {
                let a = x as i32 + i;
                let b = y as i32 + j;
                if a >= 0 && a < 10 && b >= 0 && b < 10 && board[a as usize][b as usize] != 1 {
                    let neighbor = (a as usize, b as usize);
                    neighbors.push((neighbor, 1));
                }
            }
        }
        neighbors
    };

    // Perform the A* search
    let initial_pos = snake[0];
    let goal = (9, 9);
    let path = astar(&initial_pos, |p| neighbor(p), |p| p == goal, |p| heuristic(p, &goal));

    // Return the length of the longest path to food
    path.map(|(path, _)| path.len()).unwrap_or(0)
}
```