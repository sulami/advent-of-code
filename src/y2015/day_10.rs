use crate::print_results;
use itertools::Itertools;
use std::{iter, time::Instant};

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/10").trim();
    let mut solver = iter::successors(Some(input.to_string()), |prev| Some(transform(prev)));
    let pt1 = solver.nth(40).unwrap().len();
    let pt2 = solver.nth(9).unwrap().len();
    print_results(2015, 10, pt1, pt2, Some(start));
}

fn transform(s: &str) -> String {
    s.chars()
        .dedup_with_count()
        .fold(String::new(), |acc, (cnt, ch)| acc + &format!("{cnt}{ch}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let solver = iter::successors(Some("1".to_string()), |prev| Some(transform(prev)));
        assert_eq!(
            solver.take(6).collect_vec(),
            vec!["1", "11", "21", "1211", "111221", "312211"]
        );
    }
}
