use advent_of_code_2019::*;
use std::sync::mpsc::channel;

fn main() {
    let input = get_input();
    let input: Vec<isize> = input
        .trim()
        .split(',')
        .map(|i| i.parse::<isize>().unwrap())
        .collect();

    println!("Position 0: {}", fix_program(&input));
    println!("Answer is: {}", find_proper_params(&input));
}

fn fix_program(prog: &Vec<isize>) -> isize {
    let (tx, rx) = channel();
    let mut prog = prog.clone();
    prog[1] = 12;
    prog[2] = 2;
    run_tape(&mut prog, rx, tx);
    prog[0]
}

fn find_proper_params(prog: &Vec<isize>) -> isize {
    for i in 0..99 {
        for j in 0..99 {
            let (tx, rx) = channel();
            let mut prog = prog.clone();
            prog[1] = i;
            prog[2] = j;
            run_tape(&mut prog, rx, tx);
            if prog[0] == 19690720 {
                return 100 * i + j;
            }
        }
    }

    return 0;
}
