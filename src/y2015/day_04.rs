use crate::print_results;
use rayon::prelude::*;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let secret = include_str!("inputs/04").trim();
    print_results(2015, 4, mine(secret, 5), mine(secret, 6), Some(start));
}

fn mine(secret: &str, leading_zeroes: usize) -> usize {
    (1..usize::MAX)
        .into_par_iter()
        .by_exponential_blocks()
        .find_first(|n| {
            md5(&format!("{}{}", secret, n))
                .bytes()
                .take(leading_zeroes)
                .all(|b| b == b'0')
        })
        .unwrap()
}

fn md5(s: &str) -> String {
    format!("{:x}", md5::compute(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mine() {
        assert_eq!(mine("abcdef", 5), 609043);
    }
}
