use itertools::Itertools;
use std::ops::RangeInclusive;

crate::solve!("05");

type Parsed = (Vec<RangeInclusive<usize>>, Vec<usize>);

fn parse(input: &str) -> Parsed {
    let (fresh, available) = input.split_once("\n\n").unwrap();

    let fresh = fresh
        .lines()
        .map(|line| {
            let (from, to) = line.split_once('-').unwrap();
            from.parse::<usize>().unwrap()..=to.parse().unwrap()
        })
        .collect();

    let available = available
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    (fresh, available)
}

fn part_1(x: &Parsed) -> usize {
    let (fresh, available) = x;
    available
        .iter()
        .filter(|ingredient| fresh.iter().any(|range| range.contains(ingredient)))
        .count()
}

fn part_2(x: &Parsed) -> usize {
    let (fresh, _) = x;
    fresh
        .iter()
        .sorted_unstable_by_key(|r| r.start())
        .cloned()
        .coalesce(|a, b| {
            if a.contains(b.start()) {
                Ok(*a.start()..=*a.end().max(b.end()))
            } else {
                Err((a, b))
            }
        })
        .map(|r| 1 + r.end() - r.start())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
    3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse(INPUT)), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(INPUT)), 14);
    }
}
