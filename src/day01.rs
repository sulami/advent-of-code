use itertools::Itertools;
use nom::{
    character::complete::{multispace1, u32},
    combinator::all_consuming,
    sequence::separated_pair,
    IResult,
};

super::solve!("01");

fn part_1(input: &str) -> u32 {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| parse_pair(l).expect("invalid pair").1)
        .multiunzip();
    left.iter()
        .sorted_unstable()
        .zip(right.iter().sorted_unstable())
        .map(|(x, y)| x.abs_diff(*y))
        .sum()
}

fn part_2(input: &str) -> u32 {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| parse_pair(l).expect("invalid pair").1)
        .multiunzip();
    let counts = right.iter().counts();
    left.iter()
        .map(|&x| x * *counts.get(&x).unwrap_or(&0) as u32)
        .sum()
}

fn parse_pair(s: &str) -> IResult<&str, (u32, u32)> {
    all_consuming(separated_pair(u32, multispace1, u32))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 31);
    }
}
