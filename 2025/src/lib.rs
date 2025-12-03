#![allow(clippy::unreadable_literal, clippy::must_use_candidate)]

pub mod day2;
pub mod day3;

/// Parses an array of numeric digits from `input`, with rows delimited by newlines.
pub fn parse_num_block(input: &str) -> Box<[Box<[u8]>]> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(line.bytes().map(|byte| byte - b'0').collect())
            }
        })
        .collect()
}
