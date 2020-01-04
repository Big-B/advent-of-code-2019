use std::collections::HashMap;
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

/// Run the given tape through our Intcode computer. The input is a list of integers. The list of
/// integers contains opcodes which determine what to do with the following data.
///
/// The opcode format is 'ABCDE', where:
/// DE - two-digit opcode,
/// C - mode of 1st parameter
/// B - mode of 2nd parameter
/// C - mode of 3rd parameter
///
/// Current supported opcodes (and number of parameters) are:
/// 1 (3)- Add
/// 2 (3)- Subtract
/// 3 (1)- Use input
/// 4 (1)- Produce output
/// 5 (2)- Jump if true
/// 6 (2)- Jump if false
/// 7 (3)- Less than
/// 8 (3)- Equals
/// 99 (0) - Halt
///
/// The two parameter modes are position mode (0) and immediate mode (1). Position mode indicates
/// that the parameter is a location to store/retreive the data from. Immediate mode indicates that
/// the paramater is the value to be used.
pub fn run_tape(tape: &mut [isize], input: Receiver<isize>, output: Sender<isize>) -> Vec<isize> {
    let mut tape: HashMap<usize, isize> = tape.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    let mut ip = 0;
    let mut relative_base = 0;
    let mut output_vec = Vec::new();
    loop {
        let (op_code, first_mode, second_mode, third_mode) =
            get_op_code_and_param_mode(*tape.entry(ip).or_insert(0));
        match op_code {
            99 => {
                break;
            }
            1 => {
                // Add
                // Add first two parameters and store in third
                let op1 = get_val_with_mode(&mut tape, relative_base, ip + 1, first_mode);
                let op2 = get_val_with_mode(&mut tape, relative_base, ip + 2, second_mode);

                insert_val_with_mode(&mut tape, relative_base, ip + 3, third_mode, op1 + op2);
                ip += 4;
            }
            2 => {
                // Subtract
                // Subtract first two parameters and store in third
                let op1 = get_val_with_mode(&mut tape, relative_base, ip + 1, first_mode);
                let op2 = get_val_with_mode(&mut tape, relative_base, ip + 2, second_mode);

                insert_val_with_mode(&mut tape, relative_base, ip + 3, third_mode, op1 * op2);
                ip += 4;
            }
            3 => {
                // Input
                // Take value and store in the given parameter
                let val = input.recv().unwrap();
                insert_val_with_mode(&mut tape, relative_base, ip + 1, first_mode, val);
                ip += 2;
            }
            4 => {
                // Output
                // Output the value in the given parameter
                let print = get_val_with_mode(&mut tape, relative_base, ip + 1, first_mode);

                // Don't care about this error since we'll still push to the vec
                // and return that
                let _ = output.send(print);
                output_vec.push(print);
                ip += 2;
            }
            5 => {
                // jump-if-true
                // If first parameter is non-zero, set instruction pointer to the second parameter
                if get_val_with_mode(&mut tape, relative_base, ip + 1, first_mode) != 0 {
                    ip = get_val_with_mode(&mut tape, relative_base, ip + 2, second_mode) as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                // jump-if-false
                // If first parameter is zero, set instruction pointer to the second parameter
                if get_val_with_mode(&mut tape, relative_base, ip + 1, first_mode) == 0 {
                    ip = get_val_with_mode(&mut tape, relative_base, ip + 2, second_mode) as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                // less than
                // If first parameter is less than the second, store a 1 in the third parameter,
                // otherwise 0
                let op1 = get_val_with_mode(&mut tape, relative_base, ip + 1, first_mode);
                let op2 = get_val_with_mode(&mut tape, relative_base, ip + 2, second_mode);

                insert_val_with_mode(
                    &mut tape,
                    relative_base,
                    ip + 3,
                    third_mode,
                    if op1 < op2 { 1 } else { 0 },
                );
                ip += 4;
            }
            8 => {
                // equals
                // If first parameter is equal to the second, store a 1 in the third parameter,
                // otherwise 0
                let op1 = get_val_with_mode(&mut tape, relative_base, ip + 1, first_mode);
                let op2 = get_val_with_mode(&mut tape, relative_base, ip + 2, second_mode);

                insert_val_with_mode(
                    &mut tape,
                    relative_base,
                    ip + 3,
                    third_mode,
                    if op1 == op2 { 1 } else { 0 },
                );
                ip += 4;
            }
            9 => {
                // relative base offset
                // Adjust the relative base by the value of the parameter
                let op1 = get_val_with_mode(&mut tape, relative_base, ip + 1, first_mode);
                relative_base += op1;
                ip += 2;
            }
            _ => panic!("Invalid OpCode value: {}!", op_code),
        }
    }

    output_vec
}

/// Use the mode to determine which value of the tape to use.
/// Supported modes:
/// 0 -> Position - The parameter is given as a position in the tape.
/// 1 -> Immediate - The parameter is given
/// 2 -> Relative - The parameter is relative to the relative base
pub fn get_val_with_mode(
    tape: &mut HashMap<usize, isize>,
    rb: isize,
    pos: usize,
    mode: usize,
) -> isize {
    match mode {
        0 => {
            let key = *tape.entry(pos).or_insert(0) as usize;
            *tape.entry(key).or_insert(0)
        }
        1 => *tape.entry(pos).or_insert(0),
        2 => {
            let key = *tape.entry(pos).or_insert(0);
            *tape.entry((key + rb) as usize).or_insert(0)
        }
        _ => panic!("Unsupported mode!"),
    }
}

/// Use the mode to determine which value of the tape to use.
/// Supported modes:
/// 0 -> Position - The parameter is given as a position in the tape.
/// 1 -> Immediate - The parameter is given
/// 2 -> Relative - The parameter is relative to the relative base
pub fn insert_val_with_mode(
    tape: &mut HashMap<usize, isize>,
    rb: isize,
    pos: usize,
    mode: usize,
    value: isize,
) {
    match mode {
        0 => {
            let key = *tape.entry(pos).or_insert(0) as usize;
            tape.insert(key, value);
        }
        2 => {
            let key = *tape.entry(pos).or_insert(0);
            tape.insert((key + rb) as usize, value);
        }
        _ => panic!("Unsupported mode!"),
    }
}

/// The opcode format is 'ABCDE', where:
/// DE - two-digit opcode,
/// C - mode of 1st parameter
/// B - mode of 2nd parameter
/// C - mode of 3rd parameter
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
