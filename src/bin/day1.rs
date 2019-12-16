use advent_of_code_2019::get_input;

fn main() {
    let input = get_input();
    let sum: usize = input.split_whitespace().map(|n| get_fuel_req(n.parse::<usize>().unwrap())).sum();
    println!("Sum: {}", sum);
}

fn get_fuel_req(mass: usize) -> usize {
    mass / 3 - 2
}
