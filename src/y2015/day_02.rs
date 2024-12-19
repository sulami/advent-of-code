use crate::print_results;
use nom::character::complete::{char, newline, u32};
use nom::multi::separated_list0;
use nom::sequence::{preceded, tuple};
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/02");
    let parsed = parse(input);
    let pt1 = part_1(&parsed);
    let pt2 = part_2(&parsed);
    print_results(2015, 2, pt1, pt2, Some(start));
}

fn parse(s: &str) -> Vec<(u32, u32, u32)> {
    separated_list0(
        newline::<_, ()>,
        tuple((u32, preceded(char('x'), u32), preceded(char('x'), u32))),
    )(s)
    .expect("invalid input")
    .1
}

fn part_1(packages: &[(u32, u32, u32)]) -> u32 {
    packages
        .iter()
        .map(|(l, w, h)| {
            2 * l * w + 2 * w * h + 2 * h * l + [l * w, l * h, w * h].iter().min().unwrap()
        })
        .sum()
}

fn part_2(packages: &[(u32, u32, u32)]) -> u32 {
    packages
        .iter()
        .map(|(l, w, h)| {
            [2 * l + 2 * w, 2 * l + 2 * h, 2 * w + 2 * h]
                .iter()
                .min()
                .unwrap()
                + l * w * h
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse("2x3x4")), 58);
        assert_eq!(part_1(&parse("1x1x10")), 43);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse("2x3x4")), 34);
        assert_eq!(part_2(&parse("1x1x10")), 14);
    }
}
