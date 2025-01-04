use crate::print_results;
use itertools::{Itertools, MinMaxResult};
use std::{iter, time::Instant};

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/11").trim();
    let pt1 = next_password(input);
    let pt2 = next_password(&pt1);
    print_results(2015, 11, pt1, pt2, Some(start));
}

fn next_password(current: &str) -> String {
    let mut solver = iter::successors(Some(increment(current)), |prev| Some(increment(prev)));
    solver
        .find(|p| is_valid(&p))
        .expect("no valid password")
        .to_string()
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

fn is_valid(password: &str) -> bool {
    "iol".chars().all(|c| !password.contains(c))
        && matches!(password
            .chars()
            .tuple_windows()
            .enumerate()
            .filter_map(|(idx, (a, b))| if a == b { Some(idx) } else { None })
            .minmax(), MinMaxResult::MinMax(a, b) if b - a > 1)
        && password
            .chars()
            .tuple_windows()
            .any(|(a, b, c)| ALPHABET.contains(&format!("{a}{b}{c}")))
}

fn increment(current: &str) -> String {
    let mut bytes = current.as_bytes().to_vec();
    for (idx, b) in bytes.iter_mut().enumerate().rev() {
        if *b == b'z' {
            *b = b'a';
            if idx == 0 {
                bytes.insert(0, b'a');
                break;
            }
        } else {
            *b += 1;
            break;
        }
    }
    String::from_utf8(bytes).expect("bad increment")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        assert_eq!(increment("a"), "b");
        assert_eq!(increment("ab"), "ac");
        assert_eq!(increment("az"), "ba");
        assert_eq!(increment("z"), "aa");
    }

    #[test]
    fn test_validation() {
        assert!(!is_valid("hijklmmn"));
        assert!(!is_valid("abbceffg"));
        assert!(!is_valid("abbcegjk"));
        assert!(is_valid("abcdffaa"));
        assert!(is_valid("ghjaabcc"));
    }

    #[test]
    fn test_next_password() {
        assert_eq!(next_password("abcdefgh"), "abcdffaa");
        assert_eq!(next_password("ghijklmn"), "ghjaabcc");
    }
}
