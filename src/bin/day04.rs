use advent_of_code_2019::*;

fn main() {
    let input = get_input();
    let values: Vec<usize> = input
        .trim()
        .split('-')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    println!(
        "Number of possible values: {}",
        valid_values_count(values[0], values[1])
    );
    println!(
        "Number of possible stricter values: {}",
        stricter_valid_values_count(values[0], values[1])
    );
}

fn valid_values_count(lower: usize, upper: usize) -> usize {
    let mut count = 0;
    for i in lower..=upper {
        let num = format!("{:0>6}", i);
        let num = num.as_bytes();
        let mut found_dup = false;

        for j in 0..num.len() {
            // At the last letter, so check to see if a duplicate was found
            if j == num.len() - 1 {
                // Found a duplicate somewhere so this is valid
                if found_dup {
                    count += 1;
                }
            } else {
                // Check to see if there's a duplicate with the next value
                if num[j] == num[j + 1] {
                    found_dup = true;

                // If we're larger than the next number, then this is invalid
                } else if num[j] > num[j + 1] {
                    break;
                }
            }
        }
    }
    count
}

fn stricter_valid_values_count(lower: usize, upper: usize) -> usize {
    let mut count = 0;
    for i in lower..=upper {
        let num = format!("{:0>6}", i);
        let num = num.as_bytes();
        let mut found_dup = false;

        let mut j = 0;
        loop {
            // Reached the end, break
            if j == num.len() - 1 {
                if found_dup {
                    count += 1;
                }
                break;
            }

            let mut same = 1;
            while j + same < num.len() && num[j + same] == num[j] {
                same += 1;
            }

            if j + same == num.len() {
                if same == 2 || found_dup {
                    count += 1;
                }
                break;
            }

            if num[j + same] < num[j] {
                break;
            }

            if same == 2 {
                found_dup = true;
            }

            j += same;
        }
    }
    count
}
