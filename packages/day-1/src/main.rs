use std::collections::HashMap;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));

    let input: Vec<String> = read::file(&path).lines().map(|c| c.to_string()).collect();

    println!("Day One");
    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

fn part_one(input: &[String]) -> i32 {
    input
        .into_iter()
        .filter_map(|line| {
            let numbers: String = line.chars().filter(|c| c.is_numeric()).collect();
            (numbers.chars().next().unwrap().to_string()
                + numbers.chars().last().unwrap().to_string().as_str())
            .parse::<i32>()
            .ok()
        })
        .sum::<i32>()
}

fn part_two(input: &[String]) -> i32 {
    let numbers: HashMap<&str, &str> = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
    ]);

    let shizzness: Vec<String> = input
        .into_iter()
        .map(|line| {
            let keys = numbers
                .keys()
                .map(|a| a.to_string())
                .collect::<Vec<String>>();

            let first_match = keys
                .iter()
                .filter_map(|s| line.find(s).map(|index| (s, index)))
                .min_by_key(|&(_, index)| index)
                .map(|(s, _)| String::from(s))
                .unwrap()
                .to_string();

            let last_match = keys
                .iter()
                .filter_map(|s| line.rfind(s).map(|index| (s, index)))
                .max_by_key(|&(_, index)| index)
                .map(|(s, _)| String::from(s))
                .unwrap()
                .to_string();

            numbers.get(first_match.as_str()).unwrap().to_string()
                + numbers.get(last_match.as_str()).unwrap()
        })
        .collect();

    part_one(&shizzness)
}
