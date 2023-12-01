use regex::Regex;

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
    let regex = Regex::new(
        r"oneight|threeight|nineight|twone|fiveight|eighthree|eightwo|sevenine|one|two|three|four|five|six|seven|eight|nine",
    )
    .unwrap();

    let a: Vec<String> = input
        .into_iter()
        .map(|line| {
            let v = regex.replace_all(line, |captures: &regex::Captures| {
                match captures.get(0).map_or("", |m| m.as_str()) {
                    "oneight" => "18",
                    "threeight" => "38",
                    "nineight" => "98",
                    "twone" => "21",
                    "fiveight" => "58",
                    "eighthree" => "83",
                    "eightwo" => "82",
                    "sevenine" => "79",
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    _ => "",
                }
            });

            v.to_string()
        })
        .collect();

    part_one(&a)
}
