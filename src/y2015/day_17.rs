use crate::print_results;
use itertools::Itertools;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/17");
    let containers = input
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect_vec();
    let valid_combinations = containers
        .iter()
        .powerset()
        .filter(|cs| cs.iter().copied().sum::<u32>() == 150)
        .collect_vec();
    let pt1 = valid_combinations.len();
    let pt2 = valid_combinations
        .iter()
        .min_set_by_key(|cs| cs.len())
        .len();
    print_results(2015, 17, pt1, pt2, Some(start));
}
