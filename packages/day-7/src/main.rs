use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;

struct Hand {
    score: Score,
    bid: i32,
    cards: Vec<i32>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Score {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for Hand {}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score != other.score {
            return self.score.cmp(&other.score);
        }

        self.cards
            .iter()
            .zip(&other.cards)
            .fold(Ordering::Equal, |result: Ordering, (a, b)| {
                if result != Ordering::Equal {
                    return result;
                }

                a.cmp(b)
            })
    }
}

fn main() {
    let order = [
        "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
    ];

    let score_part_one = |cards: &[i32]| -> Score {
        let mut score_counts: HashMap<i32, i32> = HashMap::new();
        for &card in cards {
            *score_counts.entry(card).or_insert(0) += 1;
        }
        let mut scores = score_counts.iter().collect::<Vec<_>>();
        scores.sort_by_key(|(_, &count)| Reverse(count));

        let max_amount = scores.first().map(|(_, &a)| a).unwrap_or(0);
        let second_max = scores.get(1).map(|(_, &a)| a).unwrap_or(0);

        match (max_amount, second_max) {
            (5, 0) => Score::FiveOfAKind,
            (4, 1) => Score::FourOfAKind,
            (3, 2) => Score::FullHouse,
            (3, _) => Score::ThreeOfAKind,
            (2, 2) => Score::TwoPair,
            (2, _) => Score::OnePair,
            _ => Score::HighCard,
        }
    };

    let order_with_joker = [
        "J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A",
    ];

    let score_part_two = |cards: &[i32]| -> Score {
        let mut score_counts: HashMap<i32, i32> = HashMap::new();
        for &card in cards {
            *score_counts.entry(card).or_insert(0) += 1;
        }
        let mut scores = score_counts
            .iter()
            .filter(|(&card, _)| card != 0)
            .collect::<Vec<_>>();
        scores.sort_by_key(|(_, &count)| Reverse(count));

        let max_amount = scores.first().map(|(_, &a)| a).unwrap_or(0);
        let second_max = scores.get(1).map(|(_, &a)| a).unwrap_or(0);
        let joker = *score_counts.get(&0).unwrap_or(&0);

        match (max_amount, second_max, joker) {
            (5, 0, 0) => Score::FiveOfAKind,
            (4, 0, 1) => Score::FiveOfAKind,
            (3, 0, 2) => Score::FiveOfAKind,
            (2, 0, 3) => Score::FiveOfAKind,
            (1, 0, 4) => Score::FiveOfAKind,
            (0, 0, 5) => Score::FiveOfAKind,
            (4, 1, 0) => Score::FourOfAKind,
            (3, 1, 1) => Score::FourOfAKind,
            (2, 1, 2) => Score::FourOfAKind,
            (1, 1, 3) => Score::FourOfAKind,
            (3, 2, 0) => Score::FullHouse,
            (2, 2, 1) => Score::FullHouse,
            (3, 1, 0) => Score::ThreeOfAKind,
            (2, 1, 1) => Score::ThreeOfAKind,
            (1, 1, 2) => Score::ThreeOfAKind,
            (2, 2, 0) => Score::TwoPair,
            (2, 1, 0) => Score::OnePair,
            (1, _, 1) => Score::OnePair,
            _ => Score::HighCard,
        }
    };

    let input = include_str!("input").lines().collect::<Vec<_>>();

    println!("Day 7");
    println!(
        "Part one: {}",
        jazz_hands(&input, order, score_part_one)
            .iter()
            .enumerate()
            .map(|(index, hand)| (index + 1) as i32 * hand.bid)
            .sum::<i32>()
    );
    println!(
        "Part two: {}",
        jazz_hands(&input, order_with_joker, score_part_two)
            .iter()
            .enumerate()
            .map(|(index, hand)| (index + 1) as i32 * hand.bid)
            .sum::<i32>()
    );
}

fn jazz_hands(input: &[&str], order: [&str; 13], calc_score: fn(&[i32]) -> Score) -> Vec<Hand> {
    let mut hands = input
        .iter()
        .map(|line| {
            let cards_with_bid = line.split_whitespace().collect::<Vec<&str>>();
            let cards = cards_with_bid
                .first()
                .unwrap()
                .chars()
                .map(|char| {
                    order
                        .iter()
                        .position(|card| card.eq(&char.to_string()))
                        .unwrap() as i32
                })
                .collect::<Vec<_>>();

            Hand {
                score: calc_score(&cards),
                bid: cards_with_bid.get(1).unwrap().parse::<i32>().unwrap(),
                cards,
            }
        })
        .collect::<Vec<Hand>>();

    hands.sort();
    hands
}
