const INPUT: &'static str = include_str!("../input.txt");

fn parse_input() -> Vec<usize> {
    INPUT.split(',')
        .map(|s| s.parse::<usize>().unwrap()).
        collect::<Vec<_>>()
}

fn main() {
    let input = parse_input();
    println!("{}", size_after_n_days(input.to_vec().as_slice(), 256))
}

const MAX_DAYS: usize = 9;

fn iterate(days: &[u64; MAX_DAYS]) -> [u64; MAX_DAYS]
{
    let mut new_days = days.clone();
    new_days.rotate_left(1);
    new_days[6] += new_days[8]; // new_days[8] is the old zero
    new_days
}

fn size_after_n_days(state: &[usize], n: i32) -> u64
{
    let mut fish_per_days: [u64; MAX_DAYS] = [0; MAX_DAYS];

    for n in state {
        fish_per_days[*n] += 1;
    }

    for _ in 0..n {
        fish_per_days = iterate(&fish_per_days);
    }
    fish_per_days.iter().sum::<u64>()
}


#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [usize; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn tests() {
        assert_eq!(size_after_n_days(TEST_INPUT.to_vec().as_slice(), 18), 26);
        assert_eq!(size_after_n_days(TEST_INPUT.to_vec().as_slice(), 80), 5934);
        assert_eq!(size_after_n_days(parse_input().as_slice(), 80), 391888);

        assert_eq!(size_after_n_days(TEST_INPUT.to_vec().as_slice(), 256), 26984457539);
        assert_eq!(size_after_n_days(parse_input().as_slice(), 256), 1754597645339);
    }
}