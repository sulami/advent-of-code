use crate::print_results;
use itertools::Itertools;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/05");
    print_results(
        2015,
        5,
        input.lines().filter(|s| is_nice(s)).count(),
        input.lines().filter(|s| is_nice_v2(s)).count(),
        Some(start),
    );
}

fn is_nice(s: &str) -> bool {
    s.chars().filter(|c| "aeiou".contains(*c)).count() >= 3
        && s.chars().tuple_windows().any(|(a, b)| a == b)
        && !["ab", "cd", "pq", "xy"].iter().any(|c| s.contains(c))
}

fn is_nice_v2(s: &str) -> bool {
    s.chars().tuple_windows().any(|(a, _, b)| a == b)
        && s.char_indices()
            .tuple_windows::<(_, _)>()
            .tuple_combinations()
            .any(|(((x, a1), (_, a2)), ((y, b1), (_, b2)))| {
                a1 == b1 && a2 == b2 && x.abs_diff(y) > 1
            })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzalrnumimnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_part_2() {
        assert!(is_nice_v2("qjhvhtzxzqqjkmpb"));
        assert!(is_nice_v2("xxyxx"));
        assert!(!is_nice_v2("uurcxstgmygtbstg"));
        assert!(!is_nice_v2("ieodomkazucvgmuy"));
        assert!(!is_nice_v2("aaa"));
    }
}
