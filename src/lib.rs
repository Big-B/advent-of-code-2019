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
    let mut ip = 0;
    loop {
        let op1 = tape[ip + 1] as usize;
        let op2 = tape[ip + 2] as usize;
        let out = tape[ip + 3] as usize;
        match tape[ip] {
            99 => break,
            1 => tape[out] = tape[op1] + tape[op2],
            2 => tape[out] = tape[op1] * tape[op2],
            _ => panic!("Invalid starting value!"),
        }
        ip += 4;
    }
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
