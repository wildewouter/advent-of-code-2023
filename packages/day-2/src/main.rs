use std::collections::HashMap;

fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));

    let input: Vec<String> = read::file(&path).lines().map(|c| c.to_string()).collect();

    let mut games: HashMap<i32, HashMap<String, i32>> = HashMap::new();

    input.iter().for_each(|s| {
        let mut parts = s.splitn(2, ":");

        let game_number = parts
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<i32>()
            .unwrap();

        let mut cubes: HashMap<String, i32> = HashMap::new();

        parts.next().unwrap().split(";").for_each(|rounds| {
            rounds.split(",").for_each(|round| {
                let mut items = round.trim().splitn(2, " ");

                let amount = items.next().unwrap().parse::<i32>().unwrap();
                let color = items.next().unwrap();

                let max_amount = cubes
                    .get(color)
                    .filter(|stored| **stored > amount)
                    .unwrap_or_else(|| &amount);

                cubes.insert(color.to_string(), max_amount.clone());
            });
        });

        games.insert(game_number, cubes);
    });

    let rules: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    println!("Day One");
    println!(
        "Part one: {}",
        games
            .iter()
            .filter_map(|(game_id, cubes)| {
                let cubes_that_work = cubes
                    .iter()
                    .filter(|(color, amount)| {
                        let rule_amount = rules.get(color.as_str()).unwrap();
                        rule_amount >= amount
                    })
                    .count();

                match cubes_that_work == cubes.iter().count() {
                    true => Some(game_id),
                    false => None,
                }
            })
            .sum::<i32>()
    );
    // println!("Part two: {}", part_two(&input));
}
