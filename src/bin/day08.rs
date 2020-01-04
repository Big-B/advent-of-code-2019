use advent_of_code_2019::*;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;

fn main() {
    let input: Vec<usize> = get_input()
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    run_test1(&input);
    run_test2(&input);
}

fn run_test1(image: &Vec<usize>) {
    let mut result: Vec<(usize, usize)> = image
        .chunks_exact(IMAGE_WIDTH * IMAGE_HEIGHT)
        .map(|v| {
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
        })
        .collect();

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

fn run_test2(image: &Vec<usize>) {
    let mut image_vec: [[Option<usize>; IMAGE_HEIGHT]; IMAGE_WIDTH] = Default::default();

    for (i, &val) in image.iter().enumerate() {
        let x = i % IMAGE_WIDTH;
        let y = (i / IMAGE_WIDTH) % IMAGE_HEIGHT;

        if image_vec[x][y] == None && val != 2 {
            image_vec[x][y] = Some(val);
        }
    }

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let out = match image_vec[i][j].unwrap() {
                0 => ' ',
                1 => 'X',
                _ => panic!("Got an invalid value!"),
            };
            print!("{} ", out);
        }
        println!("");
    }
}
