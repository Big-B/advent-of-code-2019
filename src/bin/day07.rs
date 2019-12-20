use advent_of_code_2019::*;
use std::isize;

fn main() {
    let input: Vec<isize> = get_input()
        .trim()
        .split(',')
        .map(|i| i.parse::<isize>().unwrap())
        .collect();

    run_test1(&input);
}

fn run_test1(prog: &Vec<isize>)
{
    let mut max = isize::MIN;
    let mut max_seq = (0, 0, 0, 0, 0);
    for i in 0..=4 {
        for j in 0..=4 {
            if j == i {
                continue;
            }

            for k in 0..=4 {
                if k == j || k == i {
                    continue;
                }

                for l in 0..=4 {
                    if l == k || l == j || l == i {
                        continue;
                    }

                    for m in 0..=4 {
                        if m == l || m == k || m == j || m == i {
                            continue;
                        }
                        let out = run_setting_sequence(&prog, i, j, k, l, m);
                        if out > max {
                            max = out;
                            max_seq = (i, j, k, l, m);
                        };
                    }
                }
            }
        }
    }

    println!("Max: {}", max);
    println!("Max Seq: {:?}", max_seq);
}

fn run_setting_sequence(
    prog: &Vec<isize>,
    a: isize,
    b: isize,
    c: isize,
    d: isize,
    e: isize,
) -> isize {
    let a_out = run_tape(&mut prog.clone(), vec![0, a]);
    let b_out = run_tape(&mut prog.clone(), vec![a_out[0], b]);
    let c_out = run_tape(&mut prog.clone(), vec![b_out[0], c]);
    let d_out = run_tape(&mut prog.clone(), vec![c_out[0], d]);
    run_tape(&mut prog.clone(), vec![d_out[0], e])[0]
}
