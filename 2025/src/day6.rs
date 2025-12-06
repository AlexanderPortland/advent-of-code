use itertools::Itertools;

pub fn solve(input: &str, columnwise: bool) -> u64 {
    parse_problems(input, columnwise).map(Problem::eval).sum()
}

enum Op {
    Add,
    Mul,
}

impl Op {
    fn from_str(s: &str) -> Self {
        match s {
            "*" => Op::Mul,
            "+" => Op::Add,
            unsupported => panic!("invalid operation {unsupported:?}"),
        }
    }
}

struct Problem(Vec<u64>, Op);

impl Problem {
    pub fn eval(self) -> u64 {
        let op = match self.1 {
            Op::Add => |a, b| a + b,
            Op::Mul => |a, b| a * b,
        };
        self.0.into_iter().reduce(op).unwrap()
    }
}

fn parse_problems(input: &str, columnwise: bool) -> impl Iterator<Item = Problem> {
    let mut lines = input.lines().collect::<Vec<_>>();
    let ops = parse_ops(lines.pop().unwrap());
    let vals = parse_vals(lines, columnwise);

    vals.into_iter()
        .zip_eq(ops)
        .map(|(vals, op)| Problem(vals, op))
}

fn parse_ops(last_line: &str) -> Box<[Op]> {
    last_line
        .split_whitespace()
        .map(Op::from_str)
        .collect::<Box<[Op]>>()
}

fn vals_from_chars(chars: &[Vec<Option<u8>>]) -> Vec<u64> {
    chars
        .iter()
        .map(|val_chars| {
            let mut total = 0;
            for (place, c) in val_chars.iter().rev().flatten().enumerate() {
                total += u64::from(*c) * 10_u64.pow(u32::try_from(place).unwrap());
            }
            total
        })
        .collect()
}

fn parse_vals(lines: Vec<&str>, columnwise: bool) -> Box<[Vec<u64>]> {
    if columnwise {
        let column_digits: Box<[Vec<Option<u8>>]> = lines
            .into_iter()
            .map(|row| {
                row.bytes()
                    .map(|c| vec![if c == b' ' { None } else { Some(c - b'0') }])
                    .collect::<Box<[_]>>()
            })
            .reduce(|mut a, b| {
                for i in 0..a.len() {
                    a[i].extend(b[i].iter());
                }
                a
            })
            .unwrap();

        column_digits
            .split(|columns| columns.iter().all(Option::is_none))
            .map(vals_from_chars)
            .collect()
    } else {
        lines
            .into_iter()
            .map(|row| {
                row.split_whitespace()
                    .map(|val| {
                        vec![
                            val.parse::<u64>()
                                .unwrap_or_else(|_| panic!("invalid parse of {val:?}")),
                        ]
                    })
                    .collect::<Box<[Vec<u64>]>>()
            })
            .reduce(|mut a, b| {
                for i in 0..a.len() {
                    a[i].extend(b[i].iter());
                }
                a
            })
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::solve;

    const EXAMPLE_INPUT: &str = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn pt1() {
        let example_sol = solve(EXAMPLE_INPUT, false);
        assert_eq!(example_sol, 4277556);

        let input_sol = solve(&std::fs::read_to_string("input/day6.txt").unwrap(), false);
        assert_eq!(input_sol, 6378679666679);
    }

    #[test]
    fn pt2() {
        let example_sol = solve(EXAMPLE_INPUT, true);
        assert_eq!(example_sol, 3263827);

        let input_sol = solve(&std::fs::read_to_string("input/day6.txt").unwrap(), true);
        assert_eq!(input_sol, 11494432585168);
    }
}
