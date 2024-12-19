use crate::print_results;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use rayon::prelude::*;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/19");
    let (towels, patterns) = parse(input);
    let pt1 = part_1(&towels, &patterns);
    print_results(2024, 19, pt1, 0, Some(start));
}

fn parse(s: &str) -> (Vec<&str>, Vec<&str>) {
    separated_pair(
        separated_list1(tag(", "), alpha1::<_, ()>),
        tag("\n\n"),
        separated_list1(newline, alpha1),
    )(s)
    .expect("invalid input")
    .1
}

fn part_1(towels: &[&str], patterns: &[&str]) -> usize {
    // 311 is too high.
    // 240 is too low.
    patterns
        .par_iter()
        .filter(|p| is_possible(p, towels))
        .count()
}

fn is_possible(pattern: &str, towels: &[&str]) -> bool {
    let single_stripe_towels = towels
        .iter()
        .filter(|t| t.len() == 1)
        .map(|t| t.chars().next().unwrap())
        .collect_vec();
    let multi_stripes_required = "wubrg"
        .chars()
        .filter(|s| !single_stripe_towels.contains(s))
        .collect_vec();
    let multi_towel_indices = pattern
        .char_indices()
        .filter(|(_, c)| multi_stripes_required.contains(c))
        .collect_vec();

    // For all stripes that we don't have single-stripe towels for, collect possible towel positions
    // that could cover that stripe.
    let crucial_stripes = multi_towel_indices.iter().map(|&(idx, c)|
        // Try each of the towels.
        towels.iter().flat_map(|towel|  {
            // Find all positions (anchors) of the required stripe within the towel.
            towel.chars().positions(|anchor| anchor == c).filter_map(move |anchor| {
                if anchor > idx {
                    // This towel starts before the pattern, no match.
                    return None;
                }
                if idx + towel.len() - anchor > pattern.len() {
                    // This towel ends after the pattern, no match.
                    return None;
                }
                // Align the towel with the pattern such that the anchor is on the char.
                // Check that the towel matches the pattern for its entire width.
                if pattern.chars().skip(idx - anchor).take(towel.len()).zip(towel.chars()).all(|(pc, tc)| pc == tc) {
                    Some((idx, *towel, anchor))
                } else {
                    None
                }
            })
        }).collect_vec()
    ).collect_vec();

    if crucial_stripes.iter().any(|s| s.is_empty()) {
        // No options to cover one of the stripes, not possible.
        return false;
    }

    // Check that there is a combination of towels that cover the crucial stripes without
    // overlapping through DFS.
    all_stripes_covered(None, &crucial_stripes)
}

fn all_stripes_covered(
    last_towel_end: Option<usize>,
    stripes: &[Vec<(usize, &str, usize)>],
) -> bool {
    let Some(options) = stripes.first() else {
        return true;
    };
    if last_towel_end.is_some_and(|lte| lte >= options.first().unwrap().0) {
        // This stripe already got covered by the previous towel.
        return all_stripes_covered(last_towel_end, &stripes[1..]);
    }
    options
        .iter()
        .filter(|(idx, _, anchor)| {
            last_towel_end.is_some_and(|lte| lte < idx - anchor) || last_towel_end.is_none()
        })
        .any(|(idx, towel, anchor)| {
            all_stripes_covered(Some(idx + towel.len() - anchor - 1), &stripes[1..])
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part_1() {
        let (towels, patterns) = parse(INPUT);
        assert_eq!(part_1(&towels, &patterns), 6);
    }
}
