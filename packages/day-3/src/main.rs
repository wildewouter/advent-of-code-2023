use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));

    let mut engine: HashMap<(i32, i32), char> = HashMap::new();

    read::file(&path).lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            engine.insert((x as i32, y as i32), char);
        })
    });

    let not_symbol = Regex::new(r"[0-9]|\.").unwrap();

    let mut engine_parts: Vec<i32> = Vec::new();

    engine
        .iter()
        .filter(|(_, char)| !not_symbol.is_match(&char.to_string()))
        .for_each(|((x, y), _)| {
            let mut single: HashSet<i32> = HashSet::new();

            [
                &(x - 1, *y),
                &(x + 1, *y),
                &(*x, y - 1),
                &(*x, y + 1),
                &(x + 1, y + 1),
                &(x - 1, y - 1),
                &(x - 1, y + 1),
                &(x + 1, y - 1),
            ]
            .iter()
            .filter_map(|coordinates| get_number_from_coordinate(&engine, &coordinates))
            .for_each(|a| {
                single.insert(a);
            });

            single.iter().for_each(|v| engine_parts.push(*v));
        });

    println!("Day three");
    println!("Part one: {}", engine_parts.iter().sum::<i32>());
}

fn get_number_from_coordinate(
    engine: &HashMap<(i32, i32), char>,
    (x, y): &(i32, i32),
) -> Option<i32> {
    let current = engine.get(&(*x, *y)).filter(|c| c.is_numeric());

    match current {
        None => None,
        Some(_) => {
            let left = move_left(engine, &(*x, *y), String::new());
            let right = move_right(engine, &(x + 1, *y), String::new());

            Some(left + &right)
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<i32>().unwrap())
        }
    }
}

fn move_left(engine: &HashMap<(i32, i32), char>, (x, y): &(i32, i32), tail: String) -> String {
    let numeric = engine.get(&(*x, *y)).filter(|c| c.is_numeric());

    match numeric {
        None => tail.to_string(),
        Some(adjacent_digit) => move_left(engine, &(x - 1, *y), adjacent_digit.to_string() + &tail),
    }
}

fn move_right(engine: &HashMap<(i32, i32), char>, (x, y): &(i32, i32), tail: String) -> String {
    let numeric = engine.get(&(*x, *y)).filter(|c| c.is_numeric());

    match numeric {
        None => tail.to_string(),
        Some(adjacent_digit) => {
            move_right(engine, &(x + 1, *y), tail + &adjacent_digit.to_string())
        }
    }
}
