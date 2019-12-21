use advent_of_code_2019::*;
use std::isize;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let input: Vec<isize> = get_input()
        .trim()
        .split(',')
        .map(|i| i.parse::<isize>().unwrap())
        .collect();

    run_test1(&input);
    run_test2(&input);
}

fn run_test1(prog: &Vec<isize>) {
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
    let (tx, rx) = channel();
    tx.send(a).unwrap();
    tx.send(0).unwrap();
    let a_out = run_tape(&mut prog.clone(), rx, channel().0);

    let (tx, rx) = channel();
    tx.send(b).unwrap();
    tx.send(a_out[0]).unwrap();
    let b_out = run_tape(&mut prog.clone(), rx, channel().0);

    let (tx, rx) = channel();
    tx.send(c).unwrap();
    tx.send(b_out[0]).unwrap();
    let c_out = run_tape(&mut prog.clone(), rx, channel().0);

    let (tx, rx) = channel();
    tx.send(d).unwrap();
    tx.send(c_out[0]).unwrap();
    let d_out = run_tape(&mut prog.clone(), rx, channel().0);

    let (tx, rx) = channel();
    tx.send(e).unwrap();
    tx.send(d_out[0]).unwrap();
    run_tape(&mut prog.clone(), rx, channel().0)[0]
}

fn run_test2(prog: &Vec<isize>) {
    let mut max = isize::MIN;
    let mut max_seq = (0, 0, 0, 0, 0);
    for i in 5..=9 {
        for j in 5..=9 {
            if j == i {
                continue;
            }

            for k in 5..=9 {
                if k == j || k == i {
                    continue;
                }

                for l in 5..=9 {
                    if l == k || l == j || l == i {
                        continue;
                    }

                    for m in 5..=9 {
                        if m == l || m == k || m == j || m == i {
                            continue;
                        }
                        let out = run_feedback_sequence(&prog, i, j, k, l, m);
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

fn run_feedback_sequence(
    prog: &Vec<isize>,
    a: isize,
    b: isize,
    c: isize,
    d: isize,
    e: isize,
) -> isize {
    let (a_tx, a_rx) = channel();
    let (b_tx, b_rx) = channel();
    let (c_tx, c_rx) = channel();
    let (d_tx, d_rx) = channel();
    let (e_tx, e_rx) = channel();

    // Set phase settings
    a_tx.send(a).unwrap();
    b_tx.send(b).unwrap();
    c_tx.send(c).unwrap();
    d_tx.send(d).unwrap();
    e_tx.send(e).unwrap();

    // Give each a copy of the program
    let prog_a = prog.clone();
    let prog_b = prog.clone();
    let prog_c = prog.clone();
    let prog_d = prog.clone();
    let prog_e = prog.clone();

    // Send a a zero
    a_tx.send(0).unwrap();
    let a_handle = thread::spawn(move || run_tape(&mut prog_a.clone(), a_rx, b_tx));
    let b_handle = thread::spawn(move || run_tape(&mut prog_b.clone(), b_rx, c_tx));
    let c_handle = thread::spawn(move || run_tape(&mut prog_c.clone(), c_rx, d_tx));
    let d_handle = thread::spawn(move || run_tape(&mut prog_d.clone(), d_rx, e_tx));
    let e_handle = thread::spawn(move || run_tape(&mut prog_e.clone(), e_rx, a_tx));

    a_handle.join().unwrap();
    b_handle.join().unwrap();
    c_handle.join().unwrap();
    d_handle.join().unwrap();
    e_handle.join().unwrap().pop().unwrap()
}
