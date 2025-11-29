#![feature(iterator_try_collect)]
use std::{collections::BTreeMap, path::Path};

use anyhow::Ok;
use itertools::Itertools;

#[cfg(test)]
mod test;

fn main() {
    let total = do_calc("input", pt2);
    println!("total is {total}");
}

fn do_calc(file_path: impl AsRef<Path>, pt: fn(ProcessedList, ProcessedList) -> usize) -> usize {
    let (left, right) = get_lists_from_file(file_path).unwrap();

    let (left, right) = (process_list(left), process_list(right));

    println!("lists are {left:?}");
    println!("lists are {right:?}");

    pt(left, right)
}

#[allow(dead_code)]
fn pt1(left: ProcessedList, right: ProcessedList) -> usize {
    let paired_locs = ordered_loc_ids(&left)
        .zip_eq(ordered_loc_ids(&right))
        .collect::<Vec<_>>();

    println!("paired locs are {paired_locs:?}");

    paired_locs
        .into_iter()
        .map(|(left_id, right_id)| left_id.0.abs_diff(right_id.0))
        .sum::<usize>()
}

#[allow(dead_code)]
fn pt2(left: ProcessedList, right: ProcessedList) -> usize {
    ordered_loc_ids(&left)
        .map(|left_loc| left_loc.0 * right.get(left_loc).map_or(0, Vec::len))
        .sum::<usize>()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct LocID(usize);

impl From<usize> for LocID {
    fn from(value: usize) -> Self {
        LocID(value)
    }
}

type InitialList = Box<[LocID]>;
type ProcessedList = BTreeMap<LocID, Vec<usize>>;

fn get_lists_from_file(path: impl AsRef<Path>) -> anyhow::Result<(InitialList, InitialList)> {
    let input = std::fs::read_to_string(path)?;

    // TODO: try with capacity...
    let mut left = Vec::new();
    let mut right = Vec::new();

    for row_text in input.split('\n') {
        if let [a, b] = *row_text.split_ascii_whitespace().collect::<Box<[&str]>>() {
            left.push(a.parse::<usize>()?.into());
            right.push(b.parse::<usize>()?.into());
        } else {
            panic!();
        }
    }

    Ok((left.into_boxed_slice(), right.into_boxed_slice()))
}

fn process_list(list: InitialList) -> ProcessedList {
    let mut processed: ProcessedList = BTreeMap::new();

    for (index, loc) in list.into_iter().enumerate() {
        processed.entry(loc).or_default().push(index);
    }

    processed
}

fn ordered_loc_ids(list: &ProcessedList) -> impl Iterator<Item = &LocID> {
    list.iter()
        .flat_map(|(loc, occ)| std::iter::repeat_n(loc, occ.len()))
}
