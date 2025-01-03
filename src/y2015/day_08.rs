use crate::print_results;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while_m_n},
    character::complete::char,
    multi::many0_count,
    sequence::{delimited, preceded},
};
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/08");
    let pt1 = input.lines().map(|l| l.len() - str_len(l)).sum::<usize>();
    let pt2 = input
        .lines()
        .map(|l| 2 + l.chars().filter(|c| "\\\"".contains(*c)).count())
        .sum::<usize>();
    print_results(2015, 8, pt1, pt2, Some(start));
}

fn str_len(s: &str) -> usize {
    delimited(
        char::<&str, ()>('"'),
        many0_count(alt((
            tag(r#"\\"#),
            tag(r#"\""#),
            preceded(tag(r#"\x"#), take(2_usize)),
            take_while_m_n(1, 1, |c: char| c != '"'),
        ))),
        char('"'),
    )(s)
    .expect("failed to parse string")
    .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(str_len(r#""""#), 0);
        assert_eq!(str_len(r#""abc""#), 3);
        assert_eq!(str_len(r#""aaa\"aaa""#), 7);
        assert_eq!(str_len(r#""\x27""#), 1);
    }
}
