use advent_of_code_2019::*;
use std::sync::mpsc::channel;

fn main() {
    let input: Vec<isize> = get_input()
        .trim()
        .split(',')
        .map(|i| i.parse::<isize>().unwrap())
        .collect();

    println!("Test 1: {:?}", run_test1(&input.clone()));
    println!("Test 2: {:?}", run_test2(&input.clone()));
}

fn run_test1(input: &Vec<isize>) -> Vec<isize> {
    let mut prog = input.clone();
    let (tx, rx) = channel();
    tx.send(1).unwrap();
    run_tape(&mut prog, rx, tx)
}

fn run_test2(input: &Vec<isize>) -> Vec<isize> {
    let mut prog = input.clone();
    let (tx, rx) = channel();
    tx.send(5).unwrap();
    run_tape(&mut prog, rx, tx)
}
