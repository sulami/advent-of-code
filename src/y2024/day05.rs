use std::cmp::Ordering;

use itertools::Itertools;
use nom::{
    character::complete::{char, newline, u8 as parse_u8},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

crate::solve!("05");

type Rule = (u8, u8);

fn parse(input: &str) -> (Vec<Rule>, Vec<Vec<u8>>) {
    let rules = parse_rules(input).expect("invalid rules").1;
    let manuals = input
        .lines()
        .skip(rules.len() + 1)
        .map(|l| parse_manual(l).expect("invalid manual").1)
        .collect();
    (rules, manuals)
}

fn parse_rules(s: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(newline, separated_pair(parse_u8, char('|'), parse_u8))(s)
}

fn parse_manual(s: &str) -> IResult<&str, Vec<u8>> {
    all_consuming(separated_list1(char(','), parse_u8))(s)
}

fn part_1((rules, manuals): &(Vec<Rule>, Vec<Vec<u8>>)) -> usize {
    manuals
        .iter()
        .filter(|m| is_valid(m, rules))
        .map(|m| m[m.len() / 2] as usize)
        .sum()
}

fn part_2((rules, manuals): &(Vec<Rule>, Vec<Vec<u8>>)) -> usize {
    manuals
        .iter()
        .filter(|m| !is_valid(m, rules))
        .cloned()
        .map(|mut m| {
            m.sort_by(|a, b| {
                if rules.contains(&(*a, *b)) {
                    Ordering::Less
                } else if rules.contains(&(*b, *a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });
            m
        })
        .map(|m| m[m.len() / 2] as usize)
        .sum()
}

fn is_valid(manual: &[u8], rules: &[Rule]) -> bool {
    rules.iter().all(|expected @ (a, b)| {
        if let Some(actual @ (_, _)) = manual
            .iter()
            .filter(|x| *x == a || *x == b)
            .copied()
            .collect_tuple()
        {
            return actual == *expected;
        }
        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse(INPUT)), 143);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(INPUT)), 123);
    }
}
