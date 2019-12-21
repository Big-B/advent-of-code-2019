use std::io;
use std::io::Read;
use std::sync::mpsc::{Receiver, Sender};

pub fn get_input() -> String {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf).unwrap();
    buf
}

pub fn run_tape(tape: &mut [isize], input: Receiver<isize>, output: Sender<isize>) -> Vec<isize> {
    let mut ip = 0;
    let mut output_vec = Vec::new();
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
            }
            2 => {
                // Subtract
                let op1 = get_val_with_mode(tape, ip + 1, first_mode);
                let op2 = get_val_with_mode(tape, ip + 2, second_mode);

                let out = tape[ip + 3] as usize;

                tape[out] = op1 * op2;
                ip += 4;
            }
            3 => {
                // Input
                let val = input.recv().unwrap();
                tape[tape[ip + 1] as usize] = val;
                ip += 2;
            }
            4 => {
                // Output
                let print = if first_mode == 0 {
                    tape[tape[ip + 1] as usize]
                } else {
                    tape[ip + 1]
                };

                // Don't care about this error since we'll still push to the vec
                // and return that
                let _ = output.send(print);
                output_vec.push(print);
                ip += 2;
            }
            5 => {
                // jump-if-true
                if get_val_with_mode(tape, ip + 1, first_mode) != 0 {
                    ip = get_val_with_mode(tape, ip + 2, second_mode) as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                // jump-if-false
                if get_val_with_mode(tape, ip + 1, first_mode) == 0 {
                    ip = get_val_with_mode(tape, ip + 2, second_mode) as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                // less than
                let op1 = get_val_with_mode(tape, ip + 1, first_mode);
                let op2 = get_val_with_mode(tape, ip + 2, second_mode);

                let out = tape[ip + 3] as usize;
                tape[out] = if op1 < op2 { 1 } else { 0 };
                ip += 4;
            }
            8 => {
                // equals
                let op1 = get_val_with_mode(tape, ip + 1, first_mode);
                let op2 = get_val_with_mode(tape, ip + 2, second_mode);

                let out = tape[ip + 3] as usize;
                tape[out] = if op1 == op2 { 1 } else { 0 };
                ip += 4;
            }
            _ => panic!("Invalid OpCode value: {}!", op_code),
        }
    }

    output_vec
}

pub fn get_val_with_mode(tape: &[isize], pos: usize, mode: usize) -> isize {
    if mode == 0 {
        tape[tape[pos] as usize]
    } else {
        tape[pos]
    }
}

fn get_op_code_and_param_mode(val: isize) -> (usize, usize, usize, usize) {
    let op_code = (val % 100) as usize;
    let first_mode = ((val / 100) % 10) as usize;
    let second_mode = ((val / 1000) % 10) as usize;
    let third_mode = ((val / 10_000) % 10) as usize;

    (op_code, first_mode, second_mode, third_mode)
}

pub fn get_fuel_req(mass: isize) -> isize {
    if mass < 9 {
        0
    } else {
        mass / 3 - 2
    }
}

pub fn get_full_fuel_req(mass: isize) -> isize {
    let fuel_req = get_fuel_req(mass);
    if fuel_req <= 0 {
        0
    } else {
        fuel_req + get_full_fuel_req(fuel_req)
    }
}
