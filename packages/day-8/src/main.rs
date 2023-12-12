use num::Integer;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let _example_moves = "LR".chars().collect::<Vec<_>>();
    let moves = "LLRLLRRLRLRRRLRRLLRRRLRLRLRRLRRRLRRLRLRLLRLLLRRRLRRLRRRLRRRLRRRLRLRRLLRRLRRLRRLRRRLRLRRRLLRLRRLRRRLRLRRRLRRRLRLRRRLLRRRLRRRLRLRRLRLRRRLLRRLRRLRRLRRLRLRLRRRLLRRRLRRLRRRLRLRLRRRLLRLRRLLRLRRLRLRRRLRLRRLLRRRLLRRLRLRLLRLLRRLRRLLRRLRLRRLRLRLRRRLRRLRLLLLRRLRLRLRRRLLLRRRLRRLRRLRLLRLRRRLLLRRRLRRRLRRRR".chars().collect::<Vec<_>>();

    let input = include_str!("input");

    let all_nodes = input
        .lines()
        .map(|line| Node::new(line).unwrap())
        .collect::<Vec<_>>();
    let node_map = all_nodes.iter().fold(
        HashMap::new(),
        |mut result: HashMap<String, &Node>, node| {
            result.insert(node.name.to_string(), node);
            result
        },
    );

    println!("Day 8");
    println!(
        "Part one: {}",
        &get_number_of_moves_until("AAA", |dest| dest == "ZZZ", &moves, &node_map)
    );
    let shortest_to_z = all_nodes
        .par_iter()
        .filter(|node| node.name.ends_with('Z'))
        .map(|node| {
            get_number_of_moves_until(&node.name, |dest| dest.ends_with('Z'), &moves, &node_map)
        })
        .collect::<Vec<_>>();
    println!(
        "Part two: {}",
        shortest_to_z.iter().fold(1, |acc, &next| acc.lcm(&next))
    );
}

fn get_number_of_moves_until(
    start: &str,
    is_at_destination: fn(&str) -> bool,
    directions: &[char],
    node_map: &HashMap<String, &Node>,
) -> isize {
    let mut is_at_z = false;
    let mut next = start;

    let mut count = 0;

    while !is_at_z {
        for direction in directions {
            let node = node_map.get(next).unwrap();

            next = match direction {
                'L' => node.left.as_str(),
                'R' => node.right.as_str(),
                _ => panic!("AAAAAAAAAAAAAAH"),
            };

            count += 1;

            if is_at_destination(next) {
                is_at_z = true;
                break;
            }
        }
    }
    count
}

#[derive(Clone)]
struct Node {
    name: String,
    left: String,
    right: String,
}

impl Node {
    fn new(input: &str) -> Result<Node, &str> {
        let re = Regex::new(r"(?P<first>\w{3}) = \((?P<second>\w{3}), (?P<third>\w{3})\)").unwrap();

        match re.captures(input) {
            None => Err("No matches"),
            Some(caps) => Ok(Node {
                name: String::from(caps.name("first").map_or("", |a| a.as_str())),
                left: String::from(caps.name("second").map_or("", |a| a.as_str())),
                right: String::from(caps.name("third").map_or("", |a| a.as_str())),
            }),
        }
    }
}
