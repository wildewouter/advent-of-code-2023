fn main() {
    let time_with_record = [(44, 277), (89, 1136), (96, 1890), (91, 1768)];
    // let time_with_record = [(7, 9), (15, 40), (30, 200)];

    println!("Day six");
    println!(
        "Part one: {}",
        race(&time_with_record).iter().product::<usize>()
    );

    println!(
        "Part two: {}",
        race(&[(44899691, 277113618901768)]).first().unwrap()
    );
}

fn race(the_race: &[(i64, i64)]) -> Vec<usize> {
    the_race
        .iter()
        .map(|(time, record)| {
            (1..*time)
                .filter_map(|ms| {
                    let distance = ms * (time - ms);

                    if distance > *record {
                        return Some(distance);
                    }

                    None
                })
                .count()
        })
        .collect::<Vec<usize>>()
}
