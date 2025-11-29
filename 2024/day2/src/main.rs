use std::path::Path;

#[cfg(test)]
mod test;

fn main() {
    let total = pt2("input");
    println!("total is {total}");
}

pub fn pt1(input: impl AsRef<Path>) -> usize {
    let records = parse_input(input).unwrap();

    records
        .into_iter()
        .filter(|record| record_is_safe(record, false))
        .count()
}

pub fn pt2(input: impl AsRef<Path>) -> usize {
    let records = parse_input(input).unwrap();

    records
        .into_iter()
        .filter(|record| {
            let mut rev_record = record.clone();
            rev_record.reverse();
            // This reversal is a funky way to fix things, but our solution runs into issues if skipping
            // either the first or second level is a solution, so reversing it ensures that those are also
            // always checked as last items!
            record_is_safe(record, true) || record_is_safe(&rev_record, true)
        })
        .count()
}

fn record_is_safe(record: &Record, allow_dampening: bool) -> bool {
    let mut going_up = None;
    let mut dampen_used = !allow_dampening;
    let Some(mut a) = record.first() else {
        return false;
    };

    for b in &record[1..] {
        let im_going_up = b > a;
        if let Some(going_up) = going_up
            && going_up != im_going_up
        {
            if dampen_used {
                return false;
            }
            dampen_used = true;
            continue;
        }

        going_up = Some(im_going_up);

        let diff = a.abs_diff(*b);
        if !(1..=3).contains(&diff) {
            if dampen_used {
                return false;
            }
            dampen_used = true;
            continue;
        }
        a = b;
    }

    true
}

type Record = Box<[usize]>;

fn parse_input(path: impl AsRef<Path>) -> anyhow::Result<Vec<Record>> {
    let input = std::fs::read_to_string(path)?;

    anyhow::Ok(
        input
            .split('\n')
            .filter_map(|row| {
                let row = row
                    .split_ascii_whitespace()
                    .map(|s| s.parse().expect("error parsing!"))
                    .collect::<Box<[usize]>>();
                if row.is_empty() { None } else { Some(row) }
            })
            .collect::<Vec<_>>(),
    )
}
