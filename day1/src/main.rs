use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_increases(numbers: &Vec<i32>) -> usize {
    numbers.windows(2)
        .filter(|w| w[1] > w[0])
        .count()
}

fn part_one(numbers: &Vec<i32>) {
    println!("{}", count_increases(numbers));
}

fn part_two(numbers: &Vec<i32>) {
    let windowed_sums: Vec<_> = numbers.windows(3)
        .map(|w| w[0] + w[1] + w[2]).collect();
    println!("{}", count_increases(&windowed_sums));
}

fn main() {
    let f = File::open("input.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let numbers: Vec<_> = f.lines().map(|l| l.map(|s| s.parse::<i32>().unwrap()).unwrap()).collect();
    part_one(&numbers);
    part_two(&numbers);
}