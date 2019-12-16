use std::io;
use std::io::Read;

pub fn get_input() -> String {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf).unwrap();
    buf
}

pub fn run_tape(tape: &mut [isize]) {
    let mut i = 0;
    loop {
        let op1 = tape[i + 1] as usize;
        let op2 = tape[i + 2] as usize;
        let out = tape[i + 3] as usize;
        match tape[i] {
            99 => break,
            1 => tape[out] = tape[op1] + tape[op2],
            2 => tape[out] = tape[op1] * tape[op2],
            _ => panic!("Invalid starting value!"),
        }
        i += 4;
    }
}

pub fn get_fuel_req(mass: usize) -> usize {
    mass / 3 - 2
}

pub fn get_full_fuel_req(mass: isize) -> isize {
    let fuel_req = mass / 3 - 2;
    if fuel_req <= 0 {
        0
    } else {
        fuel_req + get_full_fuel_req(fuel_req)
    }
}
