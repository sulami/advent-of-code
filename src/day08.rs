use itertools::Itertools;
use std::collections::{HashMap, HashSet};

super::solve!("08");

fn part_1(input: &str) -> usize {
    let width = input.lines().next().expect("no lines").len();
    let height = input.lines().count();
    let antennas: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.char_indices()
                .filter(|(_, c)| *c != '.')
                .map(move |(j, c)| (c, (j as isize, i as isize)))
        })
        .into_group_map();
    let antinodes: HashMap<_, HashSet<_>> = antennas
        .iter()
        .map(|(k, vs)| {
            (
                k,
                vs.iter()
                    .tuple_combinations()
                    .flat_map(|(&a, &b)| antinodes(width, height, a, b))
                    .collect(),
            )
        })
        .collect();
    antinodes.values().flatten().unique().count()
}

fn part_2(input: &str) -> usize {
    let width = input.lines().next().expect("no lines").len();
    let height = input.lines().count();
    let antennas: HashMap<_, _> = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| {
            l.char_indices()
                .filter(|(_, c)| *c != '.')
                .map(move |(j, c)| (c, (j as isize, i as isize)))
        })
        .into_group_map();
    let antinodes: HashMap<_, HashSet<_>> = antennas
        .iter()
        .map(|(k, vs)| {
            (
                k,
                vs.iter()
                    .tuple_combinations()
                    .flat_map(|(&a, &b)| antinodes_with_resonance(width, height, a, b))
                    .collect(),
            )
        })
        .collect();
    antinodes.values().flatten().unique().count()
}

type Coords = (isize, isize);

fn antinodes(width: usize, height: usize, (ax, ay): Coords, (bx, by): Coords) -> HashSet<Coords> {
    let mut nodes = HashSet::new();
    let in_bounds = |(x, y)| (0..width as isize).contains(&x) && (0..height as isize).contains(&y);
    let diff_x = ax - bx;
    let diff_y = ay - by;

    let first = (ax + diff_x, ay + diff_y);
    if in_bounds(first) {
        nodes.insert(first);
    }

    let second = (bx - diff_x, by - diff_y);
    if in_bounds(second) {
        nodes.insert(second);
    }

    nodes
}

fn antinodes_with_resonance(
    width: usize,
    height: usize,
    a @ (ax, ay): Coords,
    b @ (bx, by): Coords,
) -> HashSet<Coords> {
    let mut nodes = HashSet::new();
    let in_bounds = |(x, y)| (0..width as isize).contains(&x) && (0..height as isize).contains(&y);
    let diff_x = ax - bx;
    let diff_y = ay - by;

    let mut candidate = a;
    while in_bounds(candidate) {
        nodes.insert(candidate);
        candidate = (candidate.0 + diff_x, candidate.1 + diff_y);
    }

    candidate = b;
    while in_bounds(candidate) {
        nodes.insert(candidate);
        candidate = (candidate.0 - diff_x, candidate.1 - diff_y);
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(INPUT), 34);
    }
}
