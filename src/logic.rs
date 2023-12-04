use std::collections::HashMap;

use log::info;
use rocket::serde::json::Json;
use serde_json::{json, Value};

use crate::{Battlesnake, Board, Coord, Game};

pub fn get_info() -> Json<Value> {
    info!("INFO");
    return Json(json!({
        "apiversion": "1",
        "author": "strixos",
        "color": "#f5bf42",
        "head": "silly",
        "tail": "mlh-gene"
    }));
}

pub fn start(game: &Game, _turn: &u32, _board: &Board, _me: &Battlesnake) {
    info!("{} START", game.id);
}

pub fn end(game: &Game, _turn: &u32, _board: &Board, _me: &Battlesnake) {
    info!("{} END", game.id);
}

pub fn get_move(game: &Game, _turn: &u32, board: &Board, me: &Battlesnake) -> &'static str {
    let my_head = &me.head;

    let possible_moves: HashMap<_, _> = vec![
        ("up", value_of_move(&my_head.up(), board, me)),
        ("down", value_of_move(&my_head.down(), board, me)),
        ("left", value_of_move(&my_head.left(), board, me)),
        ("right", value_of_move(&my_head.right(), board, me)),
    ]
    .into_iter()
    .collect();

    let chosen = possible_moves.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;

    info!("{} MOVE {}", game.id, chosen);

    chosen
}

fn spot_has_food(spot: &Coord, board: &Board) -> bool {
    board.food.contains(&spot)
}

fn spot_has_hazards(spot: &Coord, board: &Board) -> bool {
    board.hazards.contains(spot)
}

fn spot_has_snake(spot: &Coord, snakes: &[Battlesnake]) -> bool {
    let mut snake_parts = vec![];
    for snake in snakes {
        snake_parts.push(snake.head);
        snake_parts.append(&mut snake.body.clone());
    }
    if snake_parts.contains(spot) {
        return true;
    }

    false
}

fn spot_might_have_snake(spot: &Coord, snakes: &[Battlesnake], me: &Battlesnake) -> bool {
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
    if snake_parts.contains(spot) {
        return true;
    }

    false
}

fn remaining_space(spot: &Coord, board: &Board, me: &Battlesnake) -> i32 {
    let available_spaces = check_spot_for_space(spot, board, me.length, [].to_vec());
    available_spaces.len() as i32
}

fn check_spot_for_space(spot: &Coord, board: &Board, my_length: i32, mut available_spaces: Vec<Coord>) -> Vec<Coord> {
    if my_length <= available_spaces.len() as i32 {
            return available_spaces;
    }
    if available_spaces.contains(spot) {
        return available_spaces;
    }

    if valid_move(spot, board) {
        available_spaces.push(*spot);

        if valid_move(&spot.right(), board) {
            available_spaces = check_spot_for_space(&spot.right(), board, my_length, available_spaces);
        }
        if valid_move(&spot.left(), board) {
            available_spaces = check_spot_for_space(&spot.left(), board, my_length, available_spaces);
        }
        if valid_move(&spot.up(), board) {
            available_spaces = check_spot_for_space(&spot.up(), board, my_length, available_spaces);
        }
        if valid_move(&spot.down(), board) {
            available_spaces = check_spot_for_space(&spot.down(), board, my_length, available_spaces);
        }
    }
    available_spaces
}

fn spot_modifier(spot: &Coord, board: &Board, me: &Battlesnake) -> i32 {
    let mut modifier = 0;
    if spot_might_have_snake(spot, &board.snakes, &me) {
        modifier -= 80;
    }
    if spot_has_food(spot, &board) {
        modifier += 75;
    } else if spot_has_hazards(spot, &board) {
        let leftover_health = me.health - 14;
        modifier -= 100 - leftover_health;
    }
    let spaces = remaining_space(spot, &board, &me);
    if spaces >= me.length {
        modifier += 50
    } else {
        modifier -= 80 - spaces
    }
    modifier
}

fn valid_move(spot: &Coord, board: &Board) -> bool {
    match spot {
        Coord { y: -1, .. } => false,
        Coord { x: -1, .. } => false,
        Coord { y, .. } if y == &board.width => false,
        Coord { x, .. } if x == &board.height => false,
        spot if spot_has_snake(spot, &board.snakes) => false,
        _ => true,
    }
}

fn value_of_move(spot: &Coord, board: &Board, me: &Battlesnake) -> i32 {
    let base_value = match spot {
        spot if spot_has_snake(spot, &board.snakes) => -10,
        spot if !valid_move(&spot, &board) => -100,
        Coord { y: 0, .. } => 60,
        Coord { x: 0, .. } => 60,
        _ => 100,
    };

    base_value + spot_modifier(spot, &board, &me)
}
