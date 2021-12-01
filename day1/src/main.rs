use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_increases(numbers: &Vec<i32>, window_size: usize) -> usize {
    numbers.windows(window_size + 1)
        .filter(|w| w[window_size] > w[0])
        .count()
}

fn part_one(numbers: &Vec<i32>) {
    println!("{}", count_increases(numbers, 1));
}

fn part_two(numbers: &Vec<i32>) {
    // (b + c + d) - (a + b + c) = d - a
    println!("{}", count_increases(numbers, 3));
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let numbers: Vec<_> = f.lines().map(|l| l.map(|s| s.parse::<i32>().unwrap()).unwrap()).collect();
    part_one(&numbers);
    part_two(&numbers);
}