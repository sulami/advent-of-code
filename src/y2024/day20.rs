use crate::coordinate::Coordinate;
use crate::{coordinate, print_results};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Instant;

pub fn solve() {
    let t = Instant::now();
    let (start, end, walls) = parse_input(include_str!("inputs/20"));
    let costs = costs(&walls, start);
    let baseline = *costs.get(&end).unwrap();
    let pt1 = cheats_saving_n_ps(100, baseline, &costs, 2);
    let pt2 = cheats_saving_n_ps(100, baseline, &costs, 20);
    print_results(2024, 20, pt1, pt2, Some(t));
}

fn cheats_saving_n_ps(
    min_savings: usize,
    baseline: usize,
    costs: &FxHashMap<Coordinate, usize>,
    max_cheat_len: usize,
) -> usize {
    costs
        .iter()
        .cartesian_product(costs.iter())
        .filter(|&((from, from_cost), (to, to_cost))| {
            if !(2..=max_cheat_len).contains(&from.manhattan_distance(*to)) {
                return false;
            }
            let new_cost = from_cost + from.manhattan_distance(*to) + to_cost.abs_diff(baseline);
            if new_cost + min_savings <= baseline {
                return true;
            }
            false
        })
        .count()
}

fn parse_input(input: &str) -> (Coordinate, Coordinate, FxHashSet<Coordinate>) {
    let (mut start, mut end) = (Coordinate::default(), Coordinate::default());
    let mut walls = FxHashSet::default();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                walls.insert(Coordinate::new(x as isize, y as isize));
            } else if ch == 'S' {
                start = Coordinate::new(x as isize, y as isize);
            } else if ch == 'E' {
                end = Coordinate::new(x as isize, y as isize);
            }
        }
    }
    (start, end, walls)
}

fn costs(walls: &FxHashSet<Coordinate>, start: Coordinate) -> FxHashMap<Coordinate, usize> {
    let mut queue: BinaryHeap<(Reverse<usize>, Coordinate)> =
        BinaryHeap::from_iter([(Reverse(0), start)]);
    let mut costs = FxHashMap::from_iter([(start, 0)]);

    while let Some((Reverse(time), position)) = queue.pop() {
        let options = coordinate::DIRECTIONS.map(|d| (d, walls.contains(&(position + d))));
        for (direction, is_wall) in options {
            let new_position = position + direction;
            if is_wall {
                continue;
            }
            if costs.contains_key(&new_position) {
                continue;
            }
            costs.insert(new_position, time + 1);
            queue.push((Reverse(time + 1), new_position));
        }
    }

    costs
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part_1() {
        let (start, end, walls) = parse_input(INPUT);
        let costs = costs(&walls, start);
        let baseline = *costs.get(&end).unwrap();
        let pt1 = cheats_saving_n_ps(2, baseline, &costs, 2);
        assert_eq!(pt1, 44);
    }

    #[test]
    fn test_part_2() {
        let (start, end, walls) = parse_input(INPUT);
        let costs = costs(&walls, start);
        let baseline = *costs.get(&end).unwrap();
        let pt2 = cheats_saving_n_ps(50, baseline, &costs, 20);
        assert_eq!(pt2, 285);
    }
}
