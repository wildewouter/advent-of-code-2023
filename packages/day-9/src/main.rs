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

    let ans = input
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
        .collect::<Vec<_>>();

    println!("Day 9");

    println!("Part one: {}", &ans.iter().sum::<isize>());
    println!("Part two: {}", "");
}
