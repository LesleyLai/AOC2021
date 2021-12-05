use std::cmp::{max, min};

const INPUT: &'static str = include_str!("../input.txt");

type Point = (isize, isize);
type Line = (Point, Point);

fn read(input: &'static str) -> Vec<Line> {
    input.lines().map(|line| {
        let mut itr = line.split(" ").map(|i| i.parse::<isize>().unwrap());
        ((itr.next().unwrap(), itr.next().unwrap()), (itr.next().unwrap(), itr.next().unwrap()))
    }).collect::<Vec<_>>()
}

struct Diagram {
    data: Vec<i32>,
}

const DIAGRAM_LENGTH: usize = 1000;

impl Diagram {
    fn new() -> Diagram {
        Diagram { data: vec![0; DIAGRAM_LENGTH * DIAGRAM_LENGTH] }
    }

    fn draw_at(&mut self, x: usize, y: usize) {
        self.data[y * DIAGRAM_LENGTH + x] += 1;
    }

    fn count_overlap(&self) -> usize {
        self.data.iter().filter(|num| **num > 1).count()
    }
}

fn part1(lines: &Vec<Line>) {
    let mut diagram = Diagram::new();

    for ((x1, y1), (x2, y2)) in lines {
        if x1 == x2 {
            let y_begin = *min(y1, y2) as usize;
            let y_end = (max(y1, y2) + 1) as usize;
            for y in y_begin..y_end {
                diagram.draw_at(*x1 as usize, y);
            }
        }
        if y1 == y2 {
            let x_begin = *min(x1, x2) as usize;
            let x_end = (max(x1, x2) + 1) as usize;
            for x in x_begin..x_end {
                diagram.draw_at(x, *y1 as usize);
            }
        }
    }

    println!("Part 1: {}", diagram.count_overlap());
}

fn part2(lines: &Vec<Line>) {
    let mut diagram = Diagram::new();

    for ((x1, y1), (x2, y2)) in lines {
        let dist = max(max(x2, x1) - min(x2, x1), max(y2, y1) - min(y2, y1)) + 1;

        let x_dir = if x2 > x1 { 1 } else if x2 < x1 { -1 } else { 0 };
        let y_dir = if y2 > y1 { 1 } else if y2 < y1 { -1 } else { 0 };

        for i in 0..dist {
            diagram.draw_at((x1 + x_dir * i) as usize, (y1 + y_dir * i) as usize);
        }
    }

    println!("Part 2: {}", diagram.count_overlap());
}

fn main() {
    let lines = read(INPUT);
    part1(&lines);
    part2(&lines);
}