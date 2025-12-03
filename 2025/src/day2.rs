use std::ops::{RangeInclusive, Rem};

pub fn solve(input: &str, many_times: bool) -> u64 {
    input
        .split(',')
        .map(range_from_str)
        .map(|range| {
            range
                .filter(|val| is_invalid(*val, many_times))
                .sum::<u64>()
        })
        .sum()
}

fn range_from_str(range: &str) -> RangeInclusive<u64> {
    let dash = range
        .find('-')
        .expect("all ranges should have a dash delimiter");

    let start = range[..dash].parse::<u64>().unwrap();
    let end = range[(dash + 1)..].parse::<u64>().unwrap();
    start..=end
}

// TODO: (opt) this generalized approach seems much slower than the original
fn is_invalid(id: u64, many_times: bool) -> bool {
    let sig_digits = id.ilog10() + 1;

    if !many_times {
        return is_split_equal(id, sig_digits, 2);
    }

    (2..=sig_digits).any(|split| is_split_equal(id, sig_digits, split))
}

fn is_split_equal(id: u64, sig: u32, split: u32) -> bool {
    if sig.rem(split) != 0 {
        return false;
    }

    let each = sig / split;

    all_equal((0..split).map(|split| {
        let start = split * each;
        let end = (split + 1) * each;
        only_digits(id, start, end)
    }))
}

fn all_equal(mut vals: impl Iterator<Item = u64>) -> bool {
    let first = vals.next().unwrap();
    vals.all(|v| v == first)
}

fn only_digits(val: u64, start: u32, end: u32) -> u64 {
    val % 10_u64.pow(end) / 10_u64.pow(start)
}

#[cfg(test)]
mod test {
    use super::solve;

    const EXAMPLE_INPUT: &str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn pt1() {
        let example_sol = solve(EXAMPLE_INPUT, false);
        assert_eq!(example_sol, 1227775554);

        let input_sol = solve(&std::fs::read_to_string("input/day2.txt").unwrap(), false);
        assert_eq!(input_sol, 28844599675);
    }

    #[test]
    fn pt2() {
        let example_sol = solve(EXAMPLE_INPUT, true);
        assert_eq!(example_sol, 4174379265);

        let input_sol = solve(&std::fs::read_to_string("input/day2.txt").unwrap(), true);
        assert_eq!(input_sol, 48778605167);
    }
}
