use rayon::prelude::*;

fn main() {
    let input = include_str!("input")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|digit| digit.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("Day 9");

    println!("Part one: {}", &sequence_it(&input).iter().sum::<isize>());
    println!(
        "Part two: {}",
        &sequence_it(
            &input
                .iter()
                .map(|seq| seq.iter().rev().cloned().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        )
        .iter()
        .sum::<isize>()
    );
}

fn sequence_it(input: &[Vec<isize>]) -> Vec<isize> {
    input
        .par_iter()
        .map(|next_sequence| {
            let mut sequences: Vec<Vec<isize>> = vec![next_sequence.clone()];

            while sequences
                .last()
                .unwrap()
                .iter()
                .filter(|a| a != &&0)
                .count()
                > 0
            {
                sequences.push(
                    sequences
                        .last()
                        .unwrap()
                        .windows(2)
                        .map(|windows| windows.get(1).unwrap() - windows.first().unwrap())
                        .collect::<Vec<_>>(),
                );
            }

            sequences
                .iter()
                .fold(0, |acc, next| acc + next.last().unwrap_or(&0))
        })
        .collect::<Vec<_>>()
}
