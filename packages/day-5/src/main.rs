use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input-example")
        .splitn(2, "\n\n")
        .collect::<Vec<&str>>();

    let seeds: Vec<i64> = input.get(0).unwrap()[7..]
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    let seed_pairs = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<(i64, i64)>>();

    let maps: HashMap<(String, String), Vec<(i64, i64, i64)>> =
        input.get(1).unwrap().split("\n\n").fold(
            HashMap::new(),
            |mut maps: HashMap<(String, String), Vec<(i64, i64, i64)>>, c| {
                let half = c.split(" map:\n").collect::<Vec<&str>>();

                let source_dest = half.get(0).unwrap().split("-to-").collect::<Vec<&str>>();

                maps.insert(
                    (
                        source_dest.get(0).unwrap().to_string(),
                        source_dest.get(1).unwrap().to_string(),
                    ),
                    half.get(1)
                        .unwrap()
                        .lines()
                        .map(|line| {
                            let items = line
                                .split_whitespace()
                                .map(|v| v.parse::<i64>().unwrap())
                                .collect::<Vec<i64>>();

                            (
                                *items.get(0).unwrap(),
                                *items.get(1).unwrap(),
                                *items.get(2).unwrap(),
                            )
                        })
                        .collect::<Vec<(i64, i64, i64)>>(),
                );

                maps
            },
        );

    println!("Day five");
    println!(
        "Part one: {}",
        seeds
            .iter()
            .map(|seed| go_to_location("seed", seed, &maps))
            .min()
            .unwrap()
    );

    println!(
        "Part two: {}",
        &seed_pairs
            .into_par_iter()
            .flat_map(|(start, end)| start..(start + end))
            .map(|seed| go_to_location("seed", &seed, &maps))
            .min()
            .unwrap()
    );
}

fn go_to_location(
    target: &str,
    number: &i64,
    maps: &HashMap<(String, String), Vec<(i64, i64, i64)>>,
) -> i64 {
    let ((_, destination), ranges) = maps
        .iter()
        .find(|((source, _), _)| source == target)
        .unwrap();

    let next_number = ranges
        .iter()
        .filter_map(|(destination, source, range)| {
            if number >= source && number <= &(source + range) {
                return Some(number - source + destination);
            }

            None
        })
        .collect::<Vec<i64>>();

    if target != "humidity" {
        return go_to_location(
            destination,
            next_number.get(0).unwrap_or_else(|| number),
            maps,
        );
    }

    *next_number.first().unwrap_or_else(|| number)
}
