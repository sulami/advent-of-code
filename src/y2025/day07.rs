use crate::coordinate::Coordinate;
use ahash::{HashMap, HashSet, HashSetExt};
use std::collections::VecDeque;

crate::solve!("07");

type Parsed = HashMap<Coordinate, char>;

fn parse(input: &str) -> Parsed {
    Coordinate::parse_grid(input, Some)
}

fn part_1(x: &Parsed) -> usize {
    let start = *x.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let mut heap = VecDeque::new();
    let mut seen = HashSet::new();
    heap.push_front(start);
    let mut splits = 0;

    while let Some(current) = heap.pop_front() {
        if seen.contains(&current) {
            continue;
        }
        seen.insert(current);
        match x.get(&current) {
            None => {}
            Some('S' | '.') => heap.push_back(current + Coordinate::new(0, 1)),
            Some('^') => {
                splits += 1;
                heap.push_back(current + Coordinate::new(-1, 0));
                heap.push_back(current + Coordinate::new(1, 0));
            }
            _ => unreachable!(),
        }
    }

    splits
}

fn part_2(x: &Parsed) -> usize {
    let start = x.iter().find(|(_, c)| **c == 'S').unwrap().0;
    trace(x, *start) + 1
}

#[memoize::memoize(CustomHasher: ahash::AHashMap, Ignore: grid)]
fn trace(grid: &Parsed, current: Coordinate) -> usize {
    match grid.get(&current) {
        None => 0,
        Some('S' | '.') => trace(grid, current + Coordinate::new(0, 1)),
        Some('^') => {
            1 + trace(grid, current + Coordinate::new(-1, 0))
                + trace(grid, current + Coordinate::new(1, 0))
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse(INPUT)), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(INPUT)), 40);
    }
}
