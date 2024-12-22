use crate::print_results;
use itertools::Itertools;
use std::iter;
use std::time::Instant;

use rayon::prelude::*;

pub fn solve() {
    let start = Instant::now();
    let input: Vec<i64> = include_str!("inputs/22")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();
    let secrets = input.iter().map(|s| secrets(*s)).collect_vec();
    let pt1: i64 = secrets.iter().map(|ss| ss.last().unwrap()).sum();
    let pcs = secrets
        .iter()
        .zip(secrets.iter().map(|ss| price_changes(ss)))
        .collect_vec();
    let mut most_bananas = 0;
    for a in -9..=9 {
        println!("{}/19", a + 10);
        for b in -9..=9 {
            for c in -9..=9 {
                for d in -9..=9 {
                    let seq = (a, b, c, d);
                    let bananas = pcs
                        .par_iter()
                        .filter_map(|(prices, diffs)| {
                            make_sale(seq, diffs).map(|idx| prices[idx] % 10)
                        })
                        .sum();
                    most_bananas = most_bananas.max(bananas);
                }
            }
        }
    }
    // 1800 is too high.
    print_results(2024, 22, pt1, most_bananas, Some(start));
}

fn make_sale(seq: (i64, i64, i64, i64), changes: &[i64]) -> Option<usize> {
    let first_match = changes
        .iter()
        .copied()
        .tuple_windows::<(_, _, _, _)>()
        .position(|window| window == seq)?;
    Some(first_match + 4)
}

fn secrets(mut init: i64) -> Vec<i64> {
    iter::from_fn(|| {
        init = next_secret(init);
        Some(init)
    })
    .take(2_000)
    .collect()
}

fn price_changes(secrets: &[i64]) -> Vec<i64> {
    secrets
        .iter()
        .map(|s| s % 10)
        .tuple_windows()
        .map(|(a, b)| a - b)
        .collect_vec()
}

fn next_secret(secret: i64) -> i64 {
    let phase_1 = ((secret * 64) ^ secret) % 16777216;
    let phase_2 = ((phase_1 / 32) ^ phase_1) % 16777216;
    ((phase_2 * 2048) ^ phase_2) % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secrets() {
        assert_eq!(next_secret(123), 15887950);
        assert_eq!(next_secret(15887950), 16495136);
    }

    #[test]
    fn test_sales() {}
}
