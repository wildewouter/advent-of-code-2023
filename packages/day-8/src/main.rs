use regex::Regex;
use std::collections::HashMap;

fn main() {
    let _example_moves = "LLR".chars().map(|a| a.to_string()).collect::<Vec<_>>();
    let moves = "LLRLLRRLRLRRRLRRLLRRRLRLRLRRLRRRLRRLRLRLLRLLLRRRLRRLRRRLRRRLRRRLRLRRLLRRLRRLRRLRRRLRLRRRLLRLRRLRRRLRLRRRLRRRLRLRRRLLRRRLRRRLRLRRLRLRRRLLRRLRRLRRLRRLRLRLRRRLLRRRLRRLRRRLRLRLRRRLLRLRRLLRLRRLRLRRRLRLRRLLRRRLLRRLRLRLLRLLRRLRRLLRRLRLRRLRLRLRRRLRRLRLLLLRRLRLRLRRRLLLRRRLRRLRRLRLLRLRRRLLLRRRLRRRLRRRR".chars().collect::<Vec<_>>();

    let input = include_str!("input");

    let nodes = input.lines().map(|line| Node::new(line).unwrap()).fold(
        HashMap::new(),
        |mut result: HashMap<String, Node>, node| {
            result.insert(node.name.to_string(), node);
            result
        },
    );

    println!("Day 8");
    println!(
        "Part one: {}",
        &get_number_of_moves_until(|dest| dest == "ZZZ", &moves, nodes)
    );
    println!("Part two: {}", "");
}

fn get_number_of_moves_until(
    is_at_destination: fn(&str) -> bool,
    directions: &[char],
    nodes: HashMap<String, Node>,
) -> i32 {
    let mut is_at_z = false;
    let mut next = String::from("AAA");

    let mut count = 0;

    while !is_at_z {
        for direction in directions {
            if is_at_destination(&next) {
                is_at_z = true;
                break;
            }

            let node = nodes.get(&next).unwrap();

            next = match direction {
                'L' => node.left.to_string(),
                'R' => node.right.to_string(),
                _ => panic!("AAAAAAAAAAAAAAH"),
            };

            count += 1;
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