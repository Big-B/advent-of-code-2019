use advent_of_code_2019::get_input;

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

fn run_tape(tape: &mut [isize]) {
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
