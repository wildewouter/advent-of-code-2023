use regex::Regex;
use std::collections::HashSet;

fn main() {
    let all_points_per_card: Vec<i32> = include_str!("input")
        .lines()
        .map(|line| {
            let card = Regex::new(r"^Card +\d+: ").unwrap();
            card.replace_all(line, "").to_string()
        })
        .map(|card_unsplit| {
            let (winning_str, playing_str) = card_unsplit.split_once(" | ").unwrap();

            let winning = all_numbers(winning_str);
            let playing = all_numbers(playing_str);

            let matches = winning
                .intersection(&playing)
                .collect::<HashSet<&i32>>()
                .len();

            Some(matches)
                .filter(|amount| amount > &0usize)
                .map(|amount| (2i32).pow((amount - 1) as u32) as i32)
                .unwrap_or_else(|| 0i32)
        })
        .collect();
    println!("Day three");
    println!("Part one: {}", all_points_per_card.iter().sum::<i32>());
    println!("Part two: {}", "");
}

fn all_numbers(playing_str: &str) -> HashSet<i32> {
    playing_str
        .replace("  ", " ")
        .trim()
        .split(" ")
        .map(|num| num.parse::<i32>().unwrap())
        .collect()
}
