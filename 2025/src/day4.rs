use crate::parse_bin_block;

pub fn solve(input: &str, remove: bool) -> u64 {
    let mut rolls = parse_bin_block(input, '@', '.');

    let mut total_removed = 0;
    loop {
        let to_remove = removeable_rolls(&rolls);
        total_removed += u64::try_from(to_remove.len()).unwrap();

        if to_remove.is_empty() || !remove {
            break;
        }

        for (x, y) in to_remove {
            rolls[y][x] = false;
        }
    }

    total_removed
}

type RollGrid = Box<[Box<[bool]>]>;

fn removeable_rolls(rolls: &RollGrid) -> Vec<(usize, usize)> {
    let mut removeable = Vec::new();
    for (y, row) in rolls.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            let adj = adjacent_nums(x, y)
                .filter(|(x, y)| {
                    rolls
                        .get(*y)
                        .is_some_and(|row| row.get(*x).copied().unwrap_or(false))
                })
                .count();
            if *val && adj < 4 {
                removeable.push((x, y));
            }
        }
    }
    removeable
}

fn adjacent_nums(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut adj = Vec::new();

    adj.extend([(x, y + 1), (x + 1, y), (x + 1, y + 1)]);
    if x >= 1 {
        adj.extend([(x - 1, y), (x - 1, y + 1)]);
    }
    if y >= 1 {
        adj.extend([(x, y - 1), (x + 1, y - 1)]);
    }
    if x >= 1 && y >= 1 {
        adj.push((x - 1, y - 1));
    }

    adj.into_iter()
}

#[cfg(test)]
mod test {
    use super::solve;

    const EXAMPLE_INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn pt1() {
        let example_sol = solve(EXAMPLE_INPUT, false);
        assert_eq!(example_sol, 13);

        let input_sol = solve(&std::fs::read_to_string("input/day4.txt").unwrap(), false);
        assert_eq!(input_sol, 1437);
    }

    #[test]
    fn pt2() {
        let example_sol = solve(EXAMPLE_INPUT, true);
        assert_eq!(example_sol, 43);

        let input_sol = solve(&std::fs::read_to_string("input/day4.txt").unwrap(), true);
        assert_eq!(input_sol, 8765);
    }
}
