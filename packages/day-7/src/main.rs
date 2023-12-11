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

    let mut hands = include_str!("input")
        .lines()
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

            let mut score_counts: HashMap<i32, i32> = HashMap::new();
            for &card in &cards {
                *score_counts.entry(card).or_insert(0) += 1;
            }
            let mut scores = score_counts.iter().collect::<Vec<_>>();
            scores.sort_by_key(|(_, &count)| Reverse(count));

            let max_amount = scores.first().map(|(_, &a)| a).unwrap_or_else(|| 0);
            let second_max = scores.get(1).map(|(_, &a)| a).unwrap_or_else(|| 0);

            let determined_score = match (max_amount, second_max) {
                (5, 0) => Score::FiveOfAKind,
                (4, 1) => Score::FourOfAKind,
                (3, 2) => Score::FullHouse,
                (3, _) => Score::ThreeOfAKind,
                (2, 2) => Score::TwoPair,
                (2, _) => Score::OnePair,
                _ => Score::HighCard,
            };

            Hand {
                score: determined_score,
                bid: cards_with_bid.get(1).unwrap().parse::<i32>().unwrap(),
                cards,
            }
        })
        .collect::<Vec<Hand>>();

    hands.sort();

    println!("Day 7");
    println!(
        "Part one: {}",
        hands
            .iter()
            .enumerate()
            .map(|(index, hand)| (index + 1) as i32 * hand.bid)
            .sum::<i32>()
    );
}
