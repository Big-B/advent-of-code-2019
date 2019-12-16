use advent_of_code_2019::*;

fn main() {
    let input = get_input();
    let mut input: Vec<isize> = input
        .trim()
        .split(',')
        .map(|i| i.parse::<isize>().unwrap())
        .collect();

    input[1] = 12;
    input[2] = 2;
    run_tape(&mut input);
    println!("Position 0: {}", input[0]);
}

