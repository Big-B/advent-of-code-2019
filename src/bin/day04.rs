use advent_of_code_2019::*;

fn main() {
    println!("Number of possible values: {}", valid_values_count(271973, 785961));
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
                if num[j] == num[j+1] {
                    found_dup = true;

                // If we're larger than the next number, then this is invalid
                } else if num[j] > num[j+1] {
                    break;
                }
            }
        }

    }
    count
}
