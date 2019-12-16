use advent_of_code_2019::get_input;

fn main() {
    let input = get_input();
    let sum: usize = input
        .split_whitespace()
        .map(|n| get_fuel_req(n.parse::<usize>().unwrap()))
        .sum();
    println!("Sum: {}", sum);
    let full_sum: isize = input
        .split_whitespace()
        .map(|n| get_full_fuel_req(n.parse::<isize>().unwrap()))
        .sum();
    println!("Full Sum: {}", full_sum);
}

fn get_fuel_req(mass: usize) -> usize {
    mass / 3 - 2
}

fn get_full_fuel_req(mass: isize) -> isize {
    let fuel_req = mass / 3 - 2;
    if fuel_req <= 0 {
        0
    } else {
        fuel_req + get_full_fuel_req(fuel_req)
    }
}
