use crate::print_results;
use ahash::AHashMap;
use itertools::Itertools;
use std::iter;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input: Vec<i64> = include_str!("inputs/22")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect_vec();
    let secrets = input.iter().map(|s| secrets(*s)).collect_vec();
    let pt1: i64 = secrets.iter().map(|ss| ss.last().unwrap()).sum();
    let pt2 = optimum_sales(&secrets);
    print_results(2024, 22, pt1, pt2, Some(start));
}

fn optimum_sales(secrets: &[Vec<i64>]) -> i64 {
    let mut sales = AHashMap::with_capacity(19_usize.pow(4));
    let prices_with_diffs = secrets
        .iter()
        .zip(secrets.iter().map(|ss| price_changes(ss)))
        .collect_vec();
    prices_with_diffs.iter().for_each(|(prices, diffs)| {
        diffs
            .iter()
            .tuple_windows::<(_, _, _, _)>()
            .enumerate()
            .unique_by(|(_, diff)| *diff)
            .for_each(|(idx, seq)| {
                sales
                    .entry(seq)
                    .and_modify(|s: &mut i64| *s += prices[idx + 4] % 10)
                    .or_insert(prices[idx + 4] % 10);
            });
    });
    *sales.values().max().expect("no sales found")
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
    fn test_sales() {
        let initial_secrets = vec![1, 2, 3, 2024];
        let secrets = initial_secrets.into_iter().map(secrets).collect_vec();
        assert_eq!(optimum_sales(&secrets), 23);
    }
}
