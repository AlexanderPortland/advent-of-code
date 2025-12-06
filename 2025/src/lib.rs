#![allow(clippy::unreadable_literal, clippy::must_use_candidate)]

pub mod day2;
pub mod day3;
pub mod day4;
// pub mod day5;
pub mod day6;

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

#[allow(clippy::used_underscore_binding)]
pub fn parse_bin_block(input: &str, _true: char, _false: char) -> Box<[Box<[bool]>]> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(
                    line.chars()
                        .map(|byte| match byte {
                            x if x == _true => true,
                            x if x == _false => false,
                            unexpected => panic!("unexpected char {unexpected:?}"),
                        })
                        .collect(),
                )
            }
        })
        .collect()
}
