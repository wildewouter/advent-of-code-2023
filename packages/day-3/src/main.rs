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

    let parts_per_position: Vec<HashSet<i32>> = engine
        .iter()
        .filter(|(_, char)| !not_symbol.is_match(&char.to_string()))
        .map(|((x, y), _)| {
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
            .fold(HashSet::new(), |mut parts_for_position, next| {
                parts_for_position.insert(next);

                parts_for_position
            })
        })
        .collect();

    println!("Day three");
    println!(
        "Part one: {}",
        parts_per_position
            .iter()
            .map(|set| set.iter().sum::<i32>())
            .sum::<i32>()
    );
    println!(
        "Part two: {}",
        parts_per_position
            .iter()
            .filter(|set| set.len() == 2)
            .map(|set| set.iter().fold(1, |acc, &part| acc * part))
            .sum::<i32>()
    );
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
