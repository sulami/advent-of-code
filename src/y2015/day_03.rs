use crate::grid::Coordinate;
use crate::print_results;
use rustc_hash::FxHashSet;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/03");
    let pt1 = part_1(input);
    let pt2 = part_2(input);
    print_results(2015, 3, pt1, pt2, Some(start));
}

fn part_1(input: &str) -> usize {
    let mut position = Coordinate::default();
    let mut visited = FxHashSet::from_iter([Coordinate::default()]);
    for c in input.chars() {
        position += direction(c);
        visited.insert(position);
    }
    visited.len()
}

fn part_2(input: &str) -> usize {
    let (mut santa, mut robo_santa) = (Coordinate::default(), Coordinate::default());
    let mut visited = FxHashSet::from_iter([Coordinate::default()]);
    for c in input.chars().step_by(2) {
        santa += direction(c);
        visited.insert(santa);
    }
    for c in input.chars().skip(1).step_by(2) {
        robo_santa += direction(c);
        visited.insert(robo_santa);
    }
    visited.len()
}

fn direction(c: char) -> Coordinate {
    match c {
        '<' => (-1, 0),
        '>' => (1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => (0, 0),
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(">"), 2);
        assert_eq!(part_1("^>v<"), 4);
        assert_eq!(part_1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("^v"), 3);
        assert_eq!(part_2("^>v<"), 3);
        assert_eq!(part_2("^v^v^v^v^v"), 11);
    }
}
