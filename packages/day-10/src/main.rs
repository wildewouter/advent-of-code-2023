use geo::{Contains, Point, Polygon};
use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let mut pipe_map: HashMap<(isize, isize), char> = HashMap::new();

    let mut position_of_s: (isize, isize) = (0, 0);

    include_str!("input")
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                pipe_map.insert((x as isize, y as isize), char);

                position_of_s = match char {
                    'S' => (x as isize, y as isize),
                    _ => position_of_s,
                };
            })
        });

    let (sx, sy) = position_of_s;

    let mut positions_of_pipe: Vec<(isize, isize)> = vec![
        position_of_s,
        *[
            ((sx - 1, sy), vec!['L', 'F', '-']),
            ((sx + 1, sy), vec!['J', '7', '-']),
            ((sx, sy + 1), vec!['|', 'F', 'J']),
            ((sx, sy - 1), vec!['|', 'L', '7']),
        ]
        .iter()
        .find(|(c, list)| list.contains(pipe_map.get(c).unwrap_or(&'P')))
        .map(|(c, _)| c)
        .unwrap(),
    ];

    let mut is_running = true;

    while is_running {
        let check = positions_of_pipe.last().unwrap();

        let next = get_next_position_in_pipe(
            check,
            positions_of_pipe.get(positions_of_pipe.len() - 2).unwrap(),
            &pipe_map,
        );

        if check == &next {
            is_running = false;
        } else {
            positions_of_pipe.push(next);
        }
    }

    println!("Day 10");
    println!("Part one: {}", (positions_of_pipe.len() - 1) / 2);
    println!("Part two: {}", ray_cast(&pipe_map, &positions_of_pipe));
}

fn ray_cast(pipe_map: &HashMap<(isize, isize), char>, pipe_coords: &[(isize, isize)]) -> usize {
    let converted_points: Vec<Point<isize>> = pipe_coords
        .iter()
        .map(|(x, y)| Point::new(*x, *y))
        .collect();

    let poly = Polygon::new(converted_points.into(), vec![]);

    pipe_map
        .keys()
        .collect::<Vec<_>>()
        .par_iter()
        .filter(|pos| !pipe_coords.contains(pos))
        .filter(|(x, y)| poly.contains(&Point::new(*x, *y)))
        .count()
}

fn get_next_position_in_pipe(
    (check_x, check_y): &(isize, isize),
    previous_position: &(isize, isize),
    map: &HashMap<(isize, isize), char>,
) -> (isize, isize) {
    let options = match map.get(&(*check_x, *check_y)).unwrap() {
        '|' => [(*check_x, check_y - 1), (*check_x, check_y + 1)],
        '-' => [(check_x - 1, *check_y), (check_x + 1, *check_y)],
        'L' => [(*check_x, check_y - 1), (check_x + 1, *check_y)],
        'J' => [(check_x - 1, *check_y), (*check_x, check_y - 1)],
        '7' => [(*check_x, check_y + 1), (check_x - 1, *check_y)],
        'F' => [(check_x + 1, *check_y), (*check_x, check_y + 1)],
        '.' => [(*check_x, *check_y), (*check_x, *check_y)],
        'S' => [(*check_x, *check_y), (*check_x, *check_y)],
        _ => panic!("AAAAAAAAAAAAAAAAAAAHHHHHHHHHHHHHHHHH"),
    };

    *options
        .iter()
        .find(|&next| next != previous_position)
        .unwrap_or(previous_position)
}
