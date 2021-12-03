const INPUT: &'static str = include_str!("../input.txt");

fn int_from_binary_str(s: &str) -> isize {
    isize::from_str_radix(s, 2).unwrap()
}

// 2035764
fn part1(lines: &[&str]) {
    let mut gamma: String = "".to_string();
    let mut epsilon: String = "".to_string();
    let bit_count = lines[0].len();
    for i in 0_usize..bit_count {
        let mut zero_count = 0;
        let mut one_count = 0;
        for line in lines {
            if line.chars().nth(i).unwrap() == '0' { zero_count += 1; } else { one_count += 1; }
        }
        gamma.push(if zero_count > one_count { '0' } else { '1' });
        epsilon.push(if zero_count > one_count { '1' } else { '0' });
    }

    println!("Part 1: {}", int_from_binary_str(gamma.as_str()) * int_from_binary_str(epsilon.as_str()));
}

fn select<'a, const SEARCH_MAX: bool>(input: &'a [&'a str], digit: usize) -> &'a [&'a str] {
    assert_ne!(input.len(), 0);
    if input.len() == 1 { return input; }

    let first_one_pos = input.iter().position(|s| s.chars().nth(digit).unwrap() == '1').unwrap();
    if SEARCH_MAX {
        if input.len() - first_one_pos >= first_one_pos { // more one or equal one
            select::<SEARCH_MAX>(&input[first_one_pos..], digit + 1)
        } else {
            select::<SEARCH_MAX>(&input[..first_one_pos], digit + 1)
        }
    } else {
        if input.len() - first_one_pos >= first_one_pos { // less zero or equal zero
            select::<SEARCH_MAX>(&input[..first_one_pos], digit + 1)
        } else {
            select::<SEARCH_MAX>(&input[first_one_pos..], digit + 1)
        }
    }
}

fn part2(lines: &[&str]) {
    let oxy = select::<true>(lines, 0).first().unwrap();
    let co2 = select::<false>(lines, 0).first().unwrap();
    println!("Part 2: {}", int_from_binary_str(oxy) * int_from_binary_str(co2));
}

fn main() {
    let mut lines: Vec<_> = INPUT.lines().collect();
    lines.sort();

    part1(lines.as_slice());
    part2(lines.as_slice());
}