use advent_of_code_2019::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = get_input();
    let paths: Vec<Vec<&str>> = input
        .split_ascii_whitespace()
        .map(|s| s.split(',').collect())
        .collect();
    let first_wire = wire_path_to_set(&paths[0]);
    let second_wire = wire_path_to_set(&paths[1]);
    println!(
        "Smallest Manhatten Intersection: {}",
        find_smallest_manhatten_intersection(
            &first_wire.keys().map(|&(x, y)| (x, y)).collect(),
            &second_wire.keys().map(|&(x, y)| (x, y)).collect()
        )
    );

    println!(
        "Smallest Steps to Intersection: {}",
        find_smallest_steps_intersection(&first_wire, &second_wire)
    );
}

fn wire_path_to_set(path: &Vec<&str>) -> HashMap<(isize, isize), usize> {
    let mut set = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut first = true;
    let mut steps = 0;

    for c in path {
        let (cmd, mag) = c.split_at(1);
        let mag = mag.parse::<isize>().unwrap();
        match cmd {
            "R" => {
                for i in x..=(x + mag) {
                    if !set.contains_key(&(i, y)) {
                        set.insert((i, y), steps);
                    }
                    steps += 1;
                }
                x += mag;
            }
            "L" => {
                for i in (x - mag)..=x {
                    if !set.contains_key(&(i, y)) {
                        set.insert((i, y), steps);
                    }
                    steps += 1;
                }
                x -= mag;
            }
            "U" => {
                for i in y..=(y + mag) {
                    if !set.contains_key(&(x, i)) {
                        set.insert((x, i), steps);
                    }
                    steps += 1;
                }
                y += mag;
            }
            "D" => {
                for i in (y - mag)..=y {
                    if !set.contains_key(&(x, i)) {
                        set.insert((x, i), steps);
                    }
                    steps += 1;
                }
                y -= mag;
            }
            _ => panic!("Unknown Command!"),
        }
        if first {
            first = false;
            set.remove(&(0, 0));
        }
    }
    set
}

fn find_smallest_manhatten_intersection(
    first: &HashSet<(isize, isize)>,
    second: &HashSet<(isize, isize)>,
) -> usize {
    first
        .intersection(second)
        .map(|(x, y)| x.abs() + y.abs())
        .min()
        .unwrap() as usize
}

fn find_smallest_steps_intersection(
    first: &HashMap<(isize, isize), usize>,
    second: &HashMap<(isize, isize), usize>,
) -> usize {
    let first_m: HashSet<(isize, isize)> = first.keys().map(|&(x, y)| (x, y)).collect();
    let second_m: HashSet<(isize, isize)> = second.keys().map(|&(x, y)| (x, y)).collect();

    first_m
        .intersection(&second_m)
        .map(|v| first.get(v).unwrap() + second.get(v).unwrap())
        .min()
        .unwrap() as usize
}
