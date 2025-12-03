use crate::parse_num_block;

pub fn solve(input: &str, turn_on: u32) -> u64 {
    parse_num_block(input)
        .iter()
        .map(|bank| max_joltage_of_bank(bank, turn_on))
        .sum()
}

fn max_joltage_of_bank(bank: impl AsRef<[u8]>, turn_on: u32) -> u64 {
    let mut remaining_bank = bank.as_ref();

    let mut total = 0;

    for i in 0..turn_on {
        let left_to_turn_on = turn_on - i - 1;
        let select_from = &remaining_bank[0..(remaining_bank.len() - (left_to_turn_on as usize))];
        let (best_digit, at_loc) = find_max(select_from).unwrap();
        total += u64::from(best_digit) * 10_u64.pow(left_to_turn_on);
        remaining_bank = &remaining_bank[(at_loc + 1)..];
    }

    total
}

fn find_max(vals: &[u8]) -> Option<(u8, usize)> {
    vals.iter()
        .enumerate()
        .rev()
        .max_by(|(_, val_a), (_, val_b)| val_a.cmp(val_b))
        .map(|(i, val)| (*val, i))
}

#[cfg(test)]
mod test {
    use crate::day3::solve;

    const EXAMPLE_INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn pt1() {
        let example_sol = solve(EXAMPLE_INPUT, 2);
        assert_eq!(example_sol, 357);

        let input_sol = solve(&std::fs::read_to_string("input/day3.txt").unwrap(), 2);
        assert_eq!(input_sol, 16812);
    }

    #[test]
    fn pt2() {
        let example_sol = solve(EXAMPLE_INPUT, 12);
        assert_eq!(example_sol, 3121910778619);

        let input_sol = solve(&std::fs::read_to_string("input/day3.txt").unwrap(), 12);
        assert_eq!(input_sol, 166345822896410);
    }
}
