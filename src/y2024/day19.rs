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
    let results: Vec<_> = patterns.par_iter().map(|p| ways(p, &towels)).collect();
    let pt1 = results.iter().filter(|n| **n > 0).count();
    let pt2: usize = results.iter().sum();
    print_results(2024, 19, pt1, pt2, Some(start));
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

fn ways(pattern: &str, towels: &[&'static str]) -> usize {
    // For all stripes, collect possible towel positions that could cover that stripe.
    let crucial_stripes = pattern.char_indices().map(|(idx, c)|
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
        return 0;
    }

    // Check that there is a combination of towels that cover the crucial stripes without
    // overlapping through DFS.
    all_stripes_covered(None, crucial_stripes)
}

#[memoize::memoize(CustomHasher: ahash::AHashMap)]
fn all_stripes_covered(
    last_towel_end: Option<usize>,
    stripes: Vec<Vec<(usize, &'static str, usize)>>,
) -> usize {
    let Some(options) = stripes.first() else {
        // Covered everything, success.
        return 1;
    };
    if last_towel_end.is_some_and(|lte| lte >= options.first().unwrap().0) {
        // This stripe already got covered by the previous towel.
        return all_stripes_covered(last_towel_end, stripes.into_iter().skip(1).collect());
    }
    options
        .iter()
        .filter(|(idx, _, anchor)| last_towel_end.is_none_or(|lte| lte < idx - anchor))
        .map(|(idx, towel, anchor)| {
            all_stripes_covered(
                Some(idx + towel.len() - anchor - 1),
                stripes.iter().skip(1).cloned().collect(),
            )
        })
        .sum()
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
        assert_eq!(patterns.iter().filter(|p| ways(p, &towels) > 0).count(), 6);
    }

    #[test]
    fn test_part_2() {
        let (towels, patterns) = parse(INPUT);
        assert_eq!(patterns.iter().map(|p| ways(p, &towels)).sum::<usize>(), 16);
    }
}
