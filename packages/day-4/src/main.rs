use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() {
    let all_winning_per_card: HashMap<i32, HashSet<i32>> = include_str!("input")
        .lines()
        .map(|line| {
            let card = Regex::new(r"^Card +\d+: ").unwrap();
            card.replace_all(line, "").to_string()
        })
        .map(|card_un_split| {
            let (winning_str, playing_str) = card_un_split.split_once(" | ").unwrap();

            let winning = all_numbers(winning_str);
            let playing = all_numbers(playing_str);
            winning
                .intersection(&playing)
                .map(|a| a.clone())
                .collect::<HashSet<i32>>()
        })
        .enumerate()
        .fold(
            HashMap::new(),
            |mut acc: HashMap<i32, HashSet<i32>>, (index, wins)| {
                acc.insert(index as i32, wins);
                acc
            },
        );
    println!("Day four");
    println!(
        "Part one: {}",
        all_winning_per_card
            .values()
            .map(|matches| {
                Some(matches.len())
                    .filter(|amount| amount > &0usize)
                    .map(|amount| (2i32).pow((amount - 1) as u32))
                    .unwrap_or_else(|| 0i32)
            })
            .sum::<i32>()
    );

    println!(
        "Part two: {}",
        all_winning_per_card
            .keys()
            .map(|card| get(card, &all_winning_per_card))
            .sum::<i32>()
            + all_winning_per_card.len() as i32
    );
}

fn get(card: &i32, winnings_per_card: &HashMap<i32, HashSet<i32>>) -> i32 {
    let no_of_wins_for_card = winnings_per_card.get(card).unwrap().len() as i32;

    let next_cards_to_check: Vec<i32> = (*card + 1..=(card + &no_of_wins_for_card)).collect();

    let sub_winnings = next_cards_to_check
        .iter()
        .map(|next_card| get(next_card, winnings_per_card))
        .sum::<i32>();

    no_of_wins_for_card + sub_winnings
}

fn all_numbers(playing_str: &str) -> HashSet<i32> {
    playing_str
        .replace("  ", " ")
        .trim()
        .split(" ")
        .map(|num| num.parse::<i32>().unwrap())
        .collect()
}
