use advent_of_code_2019::*;

fn main() {
    let input: Vec<usize> = get_input()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    run_test1(&input);
}

fn run_test1(image: &Vec<usize>) {
    const IMAGE_WIDTH: usize = 25;
    const IMAGE_HEIGHT: usize = 6;

    let mut result: Vec<(usize, usize)> = image.chunks_exact(IMAGE_WIDTH*IMAGE_HEIGHT).map(|v| {
        let mut zero = 0;
        let mut one = 0;
        let mut two = 0;
        for i in v {
            match i {
                0 => zero += 1,
                1 => one += 1,
                2 => two += 1,
                _ => panic!("Shouldn't get here!"),
            }
        }
        (zero, one * two)
    }).collect();

    let first = result.pop().unwrap();
    let mut min_zero = first.0;
    let mut min_product = first.1;
    while let Some((zeroes, product)) = result.pop() {
        if zeroes < min_zero {
            min_zero = zeroes;
            min_product = product;
        }
    }

    println!("Max Product: {}", min_product);
}
