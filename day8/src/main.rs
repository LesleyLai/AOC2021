const INPUT: &'static str = include_str!("../input.txt");


struct Entry {
    patterns: [u8; 10],
    output: [u8; 4],
}

fn pattern_to_bits(pattern: &str) -> u8 {
    let mut result: u8 = 0;
    for c in pattern.chars() {
        match c {
            'a' => result |= 0b1,
            'b' => result |= 0b10,
            'c' => result |= 0b100,
            'd' => result |= 0b1000,
            'e' => result |= 0b10000,
            'f' => result |= 0b100000,
            'g' => result |= 0b1000000,
            _ => panic!("Invalid input")
        }
    }
    result
}

fn parse_line(line: &str) -> Entry {
    let mut tokens = line.split(' ');
    let patterns = [
        tokens.next(), tokens.next(), tokens.next(), tokens.next(), tokens.next(),
        tokens.next(), tokens.next(), tokens.next(), tokens.next(), tokens.next()]
        .map(|o| o.unwrap()).map(pattern_to_bits);
    tokens.next(); // discard |
    let output = [
        tokens.next(), tokens.next(), tokens.next(), tokens.next()
    ].map(|o| o.unwrap()).map(pattern_to_bits);
    Entry { patterns, output }
}

fn count_1478(entries: &Vec<Entry>) -> i32
{
    let mut digits_count = [0; 10];
    for entry in entries {
        for pattern in entry.output {
            match pattern.count_ones()
            {
                2 => digits_count[1] += 1,
                4 => digits_count[4] += 1,
                3 => digits_count[7] += 1,
                7 => digits_count[8] += 1,
                _ => ()
            }
        }
    }
    digits_count.iter().sum::<i32>()
}

// Given an entry, find what number each pattern correspond tp
fn decode(entry: &Entry) -> [u8; 10]
{
    let mut number_to_pattern = [0; 10];

    let patterns = entry.patterns;
    for pattern in patterns {
        match pattern.count_ones()
        {
            2 => number_to_pattern[1] = pattern,
            4 => number_to_pattern[4] = pattern,
            3 => number_to_pattern[7] = pattern,
            7 => number_to_pattern[8] = pattern,
            _ => ()
        }
    }

    for pattern in patterns {
        let overlapping_with_4 = (pattern & number_to_pattern[4]).count_ones();
        let overlapping_with_1 = (pattern & number_to_pattern[1]).count_ones();
        match (pattern.count_ones(), overlapping_with_4, overlapping_with_1) {
            (6, 3, 2) => number_to_pattern[0] = pattern,
            (5, 2, 1) => number_to_pattern[2] = pattern,
            (5, 3, 2) => number_to_pattern[3] = pattern,
            (5, 3, 1) => number_to_pattern[5] = pattern,
            (6, 3, 1) => number_to_pattern[6] = pattern,
            (6, 4, 2) => number_to_pattern[9] = pattern,
            _ => ()
        }
    }

    number_to_pattern
}

fn add_output_values(entries: &Vec<Entry>) -> usize
{
    entries.iter().map(
        |entry| {
            let number_to_pattern = decode(entry);

            let digits =
                entry.output.map(|output| number_to_pattern.iter()
                    .position(|pattern| *pattern == output)
                    .unwrap());

            1000 * digits[0] + 100 * digits[1] + 10 * digits[2] + digits[3]
        },
    ).sum()
}


fn main() {
    let entries = INPUT.lines().map(parse_line).collect::<Vec<_>>();
    println!("Part 1: {}", count_1478(&entries));
    println!("Part 2: {}", add_output_values(&entries))
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = include_str!("../test_input.txt");

    #[test]
    fn part1_test()
    {
        assert_eq!(count_1478(&TEST_INPUT.lines().map(parse_line).collect::<Vec<_>>()), 26);
        assert_eq!(count_1478(&INPUT.lines().map(parse_line).collect::<Vec<_>>()), 392);
    }

    #[test]
    fn part2_test()
    {
        assert_eq!(add_output_values(&TEST_INPUT.lines().map(parse_line).collect::<Vec<_>>()), 61229);
        assert_eq!(add_output_values(&INPUT.lines().map(parse_line).collect::<Vec<_>>()), 1004688);
    }
}