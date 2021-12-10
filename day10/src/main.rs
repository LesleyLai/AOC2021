const INPUT: &'static str = include_str!("../input.txt");

fn mismatch_score(pair_type: u8) -> i64 {
    match pair_type {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => panic!("Illegal pair type {}", pair_type)
    }
}

fn first_mismatch_score(input: &[u8]) -> i64 {
    let mut stack = Vec::<u8>::new();
    for c in input {
        match c {
            b'(' => stack.push(b')'),
            b'[' => stack.push(b']'),
            b'{' => stack.push(b'}'),
            b'<' => stack.push(b'>'),
            b')' | b']' | b'}' | b'>' => match stack.last() {
                None => return mismatch_score(*c),
                Some(last) => if c == last { stack.pop(); } else { return mismatch_score(*c); }
            }
            _ => panic!("Illegal character")
        }
    }
    0
}

fn part1(input: &str) -> i64 {
    input.lines().map(|line| first_mismatch_score(line.as_bytes())).sum()
}

fn auto_complete_score(input: &[u8]) -> i64 {
    let mut stack = Vec::<u8>::new();

    for c in input {
        match c {
            b'(' => stack.push(b')'),
            b'[' => stack.push(b']'),
            b'{' => stack.push(b'}'),
            b'<' => stack.push(b'>'),
            b')' | b']' | b'}' | b'>' => match stack.last() {
                None => return -1,
                Some(last) => if c == last { stack.pop(); } else { return -1; }
            }
            _ => panic!("Illegal character")
        }
    }

    stack.iter().rev().fold(0, |acc, c| {
        let acc = acc * 5;
        match c {
            b')' => acc + 1,
            b']' => acc + 2,
            b'}' => acc + 3,
            b'>' => acc + 4,
            _ => panic!("Illegal character")
        }
    })
}

fn part2(input: &str) -> i64 {
    let mut scores = input
        .lines()
        .map(|line| auto_complete_score(line.as_bytes()))
        .filter(|score| *score > 0)
        .collect::<Vec<_>>();

    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    println!("Part 1: {}", part1(INPUT));
    println!("Part 2: {}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = include_str!("../test_input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(first_mismatch_score(b"{([(<{}[<>[]}>{[]{[(<()>"), mismatch_score(b'}'));
        assert_eq!(first_mismatch_score(b"[[<[([]))<([[{}[[()]]]"), mismatch_score(b')'));
        assert_eq!(first_mismatch_score(b"[{[{({}]{}}([{[{{{}}([]"), mismatch_score(b']'));
        assert_eq!(first_mismatch_score(b"[<(<(<(<{}))><([]([]()"), mismatch_score(b')'));
        assert_eq!(first_mismatch_score(b"<{([([[(<>()){}]>(<<{{"), mismatch_score(b'>'));
        assert_eq!(part1(TEST_INPUT), 26397);
        assert_eq!(part1(INPUT), 388713);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 288957);
        assert_eq!(part2(INPUT), 3539961434);
    }
}