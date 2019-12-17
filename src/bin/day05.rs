use std::fs;
use std::io;

fn main() {
    let input = fs::read_to_string("input/day05.txt").unwrap();
    let input: Vec<isize> = input
        .trim()
        .split(',')
        .map(|i| i.parse::<isize>().unwrap())
        .collect();

    run_test(&input.clone());
}

fn run_test(input: &Vec<isize>) {
    let mut prog = input.clone();
    run_tape(&mut prog);
}

fn run_tape(tape: &mut [isize]) {
    let mut ip = 0;
    loop {
        let (op_code, first_mode, second_mode, _third_mode) = get_op_code_and_param_mode(tape[ip]);
        match op_code {
            99 => break,
            1 => {
                // Add
                let op1 = get_val_with_mode(tape, ip + 1, first_mode);
                let op2 = get_val_with_mode(tape, ip + 2, second_mode);

                let out = tape[ip + 3] as usize;

                tape[out] = op1 + op2;
                ip += 4;
            },
            2 => {
                // Subtract
                let op1 = get_val_with_mode(tape, ip + 1, first_mode);
                let op2 = get_val_with_mode(tape, ip + 2, second_mode);

                let out = tape[ip + 3] as usize;

                tape[out] = op1 * op2;
                ip += 4;
            },
            3 => {
                // Input
                let val = get_user_input();
                tape[tape[ip + 1] as usize] = val;
                ip += 2;
            },
            4 => {
                // Output
                let print = if first_mode == 0 {
                    tape[tape[ip + 1] as usize]
                } else {
                    tape[ip + 1]
                };
                println!("{}", print);
                ip += 2;
            },
            5 => {
                // jump-if-true
                if get_val_with_mode(tape, ip + 1, first_mode) != 0
                {
                    ip = get_val_with_mode(tape, ip + 2, second_mode) as usize;
                } else {
                    ip += 3;
                }
            },
            6 => {
                // jump-if-false
                if get_val_with_mode(tape, ip + 1, first_mode) == 0
                {
                    ip = get_val_with_mode(tape, ip + 2, second_mode) as usize;
                } else {
                    ip += 3;
                }
            },
            7 => {
                // less than
                let op1 = get_val_with_mode(tape, ip + 1, first_mode);
                let op2 = get_val_with_mode(tape, ip + 2, second_mode);

                let out = tape[ip + 3] as usize;
                tape[out] = if op1 < op2 { 1 } else { 0 };
                ip += 4;
            },
            8 => {
                // equals
                let op1 = get_val_with_mode(tape, ip + 1, first_mode);
                let op2 = get_val_with_mode(tape, ip + 2, second_mode);

                let out = tape[ip + 3] as usize;
                tape[out] = if op1 == op2 { 1 } else { 0 };
                ip += 4;
            },
            _ => panic!("Invalid starting value!"),
        }
    }
}

fn get_val_with_mode(tape: &[isize], pos: usize, mode: usize) -> isize {
    if mode == 0 {
        tape[tape[pos] as usize]
    } else {
        tape[pos]
    }
}

fn get_user_input() -> isize {
    println!("Input Number: ");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    input_text.trim().parse::<isize>().unwrap()
}

fn get_op_code_and_param_mode(val: isize) -> (usize, usize, usize, usize) {
    let op_code = (val % 100) as usize;
    let first_mode = ((val / 100) % 10) as usize;
    let second_mode = ((val / 1000) % 10) as usize;
    let third_mode = ((val / 10_000) % 10) as usize;

    (op_code, first_mode, second_mode, third_mode)
}
