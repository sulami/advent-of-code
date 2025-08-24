use crate::print_results;
use itertools::Itertools;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/19");
    let replacements = input.split_once("\n\n").unwrap().0.lines()
        .map(|l| {
            let mut elems = l.split_whitespace();
            (elems.next().unwrap(), elems.nth(1).unwrap())
        })
        .collect_vec();
    let molecule = input.split_once("\n\n").unwrap().1;
    let pt1 = 1;
    let pt2 = 2;
    print_results(2015, 19, pt1, pt2, Some(start));
}

