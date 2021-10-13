use rocket_contrib::json::JsonValue;
use std::collections::HashMap;

use log::info;

use crate::{Battlesnake, Board, Coord, Game};

pub fn get_info() -> JsonValue {
    info!("INFO");

    // Personalize the look of your snake per https://docs.battlesnake.com/references/personalization
    return json!({
        "apiversion": "1",
        "author": "ChaelCodes",
        "color": "#F09383",
        "head": "bendr",
        "tail": "round-bum",
    });
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _me: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _me: &Battlesnake) {
    info!("{} END", game.id);
}

pub fn get_move(game: &Game, _turn: &u32, board: &Board, me: &Battlesnake) -> &'static str {
    let mut possible_moves: HashMap<_, _> = vec![
        ("up", 50),
        ("down", 50),
        ("left", 50),
        ("right", 50),
    ]
    .into_iter()
    .collect();

    // Step 0: Don't let your Battlesnake move back in on its own neck
    let my_head = &me.head;

    // Use board information to prevent your Battlesnake from moving beyond the boundaries of the board.

    possible_moves.insert("left", value_of_move(&my_head.left(), &board, &me));
    possible_moves.insert("right", value_of_move(&my_head.right(), &board, &me));
    possible_moves.insert("up", value_of_move(&my_head.up(), &board, &me));
    possible_moves.insert("down", value_of_move(&my_head.down(), &board, &me));

    // TODO: Step 4 - Find food.
    // Use board information to seek out and find food.
    // food = move_req.board.food

    // Finally, choose a move from the available safe moves.
    // TODO: Step 5 - Select a move to make based on strategy, rather than random.
    let chosen = possible_moves.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;

    info!("{} MOVE {}", game.id, chosen);

    return chosen;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_move() {
        let head = Coord { x: 9, y: 9 };
        let me = Battlesnake {
            body: vec![
                head.left()
            ],
            head: head,
            ..Default::default()
        };
        let board = Board {
            height: 10,
            width: 10,
            snakes: vec![me.clone()],
            ..Default::default()
        };
        let game = Game::default();
        let get_move = get_move(&game, &0, &board, &me);
        assert_eq!(get_move, "down");
    }
}

fn spot_has_hazards(spot: &Coord, board: &Board) -> bool {
    board.hazards.contains(&spot)
}

#[cfg(test)]
mod spot_has_hazards_test {
    use super::*;

    #[test]
    fn hazardous_spot_test() {
        let board = Board {
            hazards: vec![
                Coord { x: 0, y: 0},
                Coord { x: 0, y: 1},
                Coord { x: 0, y: 2},
                Coord { x: 0, y: 3},
                Coord { x: 0, y: 4},
                Coord { x: 0, y: 5},
                Coord { x: 0, y: 6},
                Coord { x: 0, y: 7},
                Coord { x: 0, y: 8},
                Coord { x: 0, y: 9},
            ],
            ..Default::default()
        };
        let spot = Coord { x: 0, y: 5 };
        assert_eq!(spot_has_hazards(&spot, &board), true);
    }

    #[test]
    fn safe_spot_test() {
        let board = Board {
            hazards: vec![
                Coord { x: 0, y: 0},
                Coord { x: 0, y: 1},
                Coord { x: 0, y: 2},
                Coord { x: 0, y: 3},
                Coord { x: 0, y: 4},
                Coord { x: 0, y: 5},
                Coord { x: 0, y: 6},
                Coord { x: 0, y: 7},
                Coord { x: 0, y: 8},
                Coord { x: 0, y: 9},
            ],
            ..Default::default()
        };
        let spot = Coord { x: 3, y: 5 };
        assert_eq!(spot_has_hazards(&spot, &board), false);
    }
}

fn spot_has_snake(spot: &Coord, snakes: &Vec<Battlesnake>) -> bool {
    let mut snake_parts = vec![];
    for snake in snakes {
        snake_parts.push(snake.head);
        snake_parts.append(&mut snake.body.clone());
    }
    if snake_parts.contains(&spot) {
        return true;
    }

    false
}

#[cfg(test)]
mod spot_has_snake_tests {
    use super::*;

    #[test]
    fn no_snakes_in_spot() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![
                Coord { x: 3, y: 5 },
                Coord { x: 4, y: 5 },
                Coord { x: 5, y: 5 },
            ],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 5, y: 7 };
        assert_eq!(spot_has_snake(&spot, &snakes), false);
    }

    #[test]
    fn head_in_spot() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            head: Coord { x: 2, y: 3 },
            body: vec![Coord { x: 3, y: 3 }, Coord { x: 3, y: 2 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 2, y: 3 };
        assert_eq!(spot_has_snake(&spot, &snakes), true);
    }

    #[test]
    fn tail_in_spot() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            head: Coord { x: 2, y: 3 },
            body: vec![Coord { x: 3, y: 3 }, Coord { x: 3, y: 2 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 3, y: 2 };
        assert_eq!(spot_has_snake(&spot, &snakes), true);
    }

    #[test]
    fn hettie_is_in_spot() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![
                Coord { x: 3, y: 5 },
                Coord { x: 4, y: 5 },
                Coord { x: 5, y: 5 },
            ],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 0, y: 0 };
        assert_eq!(spot_has_snake(&spot, &snakes), true);
    }

    #[test]
    fn i_am_in_spot() {
        let me = Battlesnake {
            name: "CorneliusCodes".to_string(),
            body: vec![
                Coord { x: 3, y: 5 },
                Coord { x: 4, y: 5 },
                Coord { x: 5, y: 5 },
            ],
            ..Default::default()
        };
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 0, y: 0 }, Coord { x: 1, y: 0 }],
            ..Default::default()
        };
        let snakes = vec![hettie, me];
        let spot = Coord { x: 5, y: 5 };
        assert_eq!(spot_has_snake(&spot, &snakes), true);
    }
}

fn spot_might_have_snake(spot: &Coord, snakes: &Vec<Battlesnake>, me: &Battlesnake) -> bool {
    let mut snake_parts = vec![];
    for snake in snakes {
        if snake.id != me.id && snake.length >= me.length {
            let head = snake.head;

            snake_parts.push(head.left());
            snake_parts.push(head.right());
            snake_parts.push(head.up());
            snake_parts.push(head.down());
        }
    }
    if snake_parts.contains(&spot) {
        return true;
    }

    false
}

#[cfg(test)]
mod spot_might_have_snake_tests {
    use super::*;

    #[test]
    fn no_snakes_in_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "Hettie".to_string(),
            head: Coord { x: 0, y: 0 },
            length: 4,
            ..Default::default()
        };
        let snakes = vec![hettie.clone(), me.clone()];
        let spot = Coord { x: 5, y: 7 };
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), false);
    }

    #[test]
    fn larger_snake_head_right_of_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: head,
            length: 4,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.right();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), true);
    }

    #[test]
    fn larger_snake_head_left_of_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: head,
            length: 4,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.left();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), true);
    }

    #[test]
    fn same_size_snake_head_above_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: head,
            length: 3,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.down();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), true);
    }

    #[test]
    fn same_size_snake_head_below_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 3,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: head,
            length: 3,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.up();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), true);
    }

    #[test]
    fn smaller_snake_head_next_to_spot() {
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: Coord { x: 7, y: 6 },
            length: 4,
            ..Default::default()
        };
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: head,
            length: 3,
            ..Default::default()
        };
        let snakes = vec![hettie];
        let spot = head.right();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), false);
    }

    #[test]
    fn i_am_next_to_spot() {
        let head = Coord { x: 3, y: 5 };
        let me = Battlesnake {
            id: "me".to_string(),
            name: "CorneliusCodes".to_string(),
            head: head,
            ..Default::default()
        };
        let snakes = vec![me.clone()];
        let spot = head.right();
        assert_eq!(spot_might_have_snake(&spot, &snakes, &me), false);
    }
}

// Returns the potential value of the move Cornelius
fn value_of_move(spot: &Coord, board: &Board, me: &Battlesnake) -> u32 {
    let board_width = board.width;
    let board_height = board.height;

    match spot {
        Coord { y: 0, .. } => 0,
        Coord { x: 0, .. } => 0,
        Coord { y, .. } if y == &board_width => 0, // Rust is weird
        Coord { x, .. } if x == &board_height => 0,
        spot if spot_has_snake(spot, &board.snakes) => 0,
        spot if spot_might_have_snake(spot, &board.snakes, &me) => 25,
        spot if spot_has_hazards(spot, &board) => &me.health - 14,
        _ => 100,
    }
}

#[cfg(test)]
mod value_of_move_tests {
    use super::*;

    // Wall Tests
    #[test]
    fn head_will_not_hit_left_wall() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 0, y: 5 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, 0);
    }

    #[test]
    fn head_will_not_hit_right_wall() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 10, y: 5 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, 0);
    }

    #[test]
    fn head_will_not_hit_roof() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: 10 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, 0);
    }

    #[test]
    fn head_will_not_hit_floor() {
        let me = Battlesnake {
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: 0 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, 0);
    }

    // Collision Tests

    #[test]
    fn do_not_hit_me() {
        let me = Battlesnake {
            body: vec![Coord { x: 5, y: 4 }, Coord { x: 5, y: 5 }],
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: 5 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, 0);
    }

    #[test]
    fn do_not_bite_hettie() {
        let me = Battlesnake::default();
        let hettie = Battlesnake {
            name: "Hettie".to_string(),
            body: vec![Coord { x: 3, y: 2 }, Coord { x: 4, y: 2 }],
            ..Default::default()
        };
        let board = Board {
            snakes: vec![hettie, me.clone()],
            ..Default::default()
        };
        let spot = Coord { x: 4, y: 2 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, 0);
    }

    #[test]
    fn potential_snake_head() {
        let me = Battlesnake::default();
        let head = Coord { x: 3, y: 5 };
        let hettie = Battlesnake {
            id: "hettie".to_string(),
            name: "HettieCodes".to_string(),
            head: head,
            length: 4,
            ..Default::default()
        };
        let spot = head.right();
        let board = Board {
            snakes: vec![me.clone(), hettie],
            ..Default::default()
        };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, 25);
    }

    #[test]
    fn hazards_identified() {
        let me = Battlesnake {
            health: 65 + 14,
            ..Default::default()
        };
        let board = Board {
            hazards: vec![
                Coord { x: 10, y: 0},
                Coord { x: 10, y: 1},
                Coord { x: 10, y: 2},
                Coord { x: 10, y: 3},
                Coord { x: 10, y: 4},
                Coord { x: 10, y: 5},
                Coord { x: 10, y: 6},
                Coord { x: 10, y: 7},
                Coord { x: 10, y: 8},
                Coord { x: 10, y: 9},
            ],
            ..Default::default()
        };
        let spot = Coord { x: 10, y: 7 };
        let value_of_move = value_of_move(&spot, &board, &me);
        assert_eq!(value_of_move, 65);
    }

    #[test]
    fn head_will_travel() {
        let me = Battlesnake {
            body: vec![Coord { x: 5, y: 9 }, Coord { x: 5, y: 8 }],
            ..Default::default()
        };
        let board = Board {
            width: 10,
            height: 10,
            food: vec![],
            hazards: vec![],
            snakes: vec![me.clone()],
        };
        let spot = Coord { x: 5, y: 5 };
        let valid_move = value_of_move(&spot, &board, &me);
        assert_eq!(valid_move, 100);
    }
}
