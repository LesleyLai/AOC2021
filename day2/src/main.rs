const INPUT: &'static str = include_str!("../input.txt");

struct Submarine {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Submarine {
    fn new() -> Submarine {
        Submarine { horizontal: 0, depth: 0, aim: 0 }
    }
}

fn parse_commands() -> Vec<(&'static str, i32)>
{
    INPUT.lines()
        .map(|l| l.split(' '))
        .map(|mut tokens| (tokens.next().unwrap(), tokens.next().unwrap().parse::<i32>().unwrap()))
        .collect()
}

fn part1(commands: &Vec<(&'static str, i32)>) {
    let result =
        commands.iter()
            .fold(Submarine::new(), |acc, (dir, distance)|
                match dir {
                    &"forward" => Submarine { horizontal: acc.horizontal + distance, ..acc },
                    &"backward" => Submarine { horizontal: acc.horizontal - distance, ..acc },
                    &"up" => Submarine { depth: acc.depth - distance, ..acc },
                    &"down" => Submarine { depth: acc.depth + distance, ..acc },
                    _ => panic!("Format error!")
                });
    println!("{}", result.depth * result.horizontal)
}

fn part2(commands: &Vec<(&'static str, i32)>) {
    let result =
        commands.iter()
            .fold(Submarine::new(), |acc, (dir, distance)|
                match dir {
                    &"forward" =>
                        Submarine {
                            horizontal: acc.horizontal + distance,
                            depth: acc.depth + acc.aim * distance,
                            ..acc
                        },
                    &"backward" =>
                        Submarine {
                            horizontal: acc.horizontal - distance,
                            depth: acc.depth - acc.aim * distance,
                            ..acc
                        },
                    &"up" => Submarine { aim: acc.aim - distance, ..acc },
                    &"down" => Submarine { aim: acc.aim + distance, ..acc },
                    _ => panic!("Format error!")
                });
    println!("{}", result.depth * result.horizontal)
}

fn main() {
    let commands = parse_commands();
    part1(&commands);
    part2(&commands);
}
