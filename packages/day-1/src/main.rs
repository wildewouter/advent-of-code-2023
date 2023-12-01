fn main() {
    let path = format!("{}/input", env!("CARGO_MANIFEST_DIR"));

    let lines: Vec<String> = read::file(&path).lines().map(|c| c.to_string()).collect();

    let summed_calibrations = lines
        .into_iter()
        .filter_map(|line| {
            let numbers: String = line.chars().filter(|c| c.is_numeric()).collect();
            (numbers.chars().next().unwrap().to_string()
                + numbers.chars().last().unwrap().to_string().as_str())
            .parse::<i32>()
            .ok()
        })
        .sum::<i32>();

    println!("Day One");
    println!("Part one: {}", &summed_calibrations);
}
