use itertools::Itertools;
use nom::{
    character::complete::{multispace1, u32 as parse_u32},
    combinator::all_consuming,
    sequence::separated_pair,
};

super::solve!("01");

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut parse_pair = all_consuming(separated_pair(
        parse_u32::<&str, ()>,
        multispace1,
        parse_u32,
    ));
    input
        .lines()
        .map(|l| parse_pair(l).expect("invalid pair").1)
        .multiunzip()
}

fn part_1((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    left.iter()
        .sorted_unstable()
        .zip(right.iter().sorted_unstable())
        .map(|(x, y)| x.abs_diff(*y))
        .sum()
}

fn part_2((left, right): &(Vec<u32>, Vec<u32>)) -> u32 {
    let counts = right.iter().counts();
    left.iter()
        .map(|&x| x * *counts.get(&x).unwrap_or(&0) as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse(INPUT)), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(INPUT)), 31);
    }
}
