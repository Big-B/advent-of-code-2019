use advent_of_code_2019::*;

fn main() {
    let input = get_input();
    let sum: isize = input
        .split_whitespace()
        .map(|n| get_fuel_req(n.parse::<isize>().unwrap()))
        .sum();
    println!("Sum: {}", sum);
    let full_sum: isize = input
        .split_whitespace()
        .map(|n| get_full_fuel_req(n.parse::<isize>().unwrap()))
        .sum();
    println!("Full Sum: {}", full_sum);
}
