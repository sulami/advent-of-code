use crate::grid::Coordinate;
use crate::print_results;
use nom::InputIter;
use rustc_hash::FxHashSet;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let pt1 = part_1(include_str!("inputs/20"));
    let pt2 = 0;
    print_results(2024, 20, pt1, pt2, Some(start));
}

fn part_1(input: &str) -> usize {
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
    let baseline = shortest_route(&walls, start, end).unwrap();
    let mut skips = 0;
    for skip in walls.clone().iter() {
        if !skip.is_in_bounds(1..input.position(|c| c == '\n').unwrap() as isize - 1) {
            continue;
        }
        walls.remove(skip);
        if let Some(skip_time) = shortest_route(&walls, start, end) {
            if baseline - skip_time >= 100 {
                skips += 1;
            }
        }
        walls.insert(*skip);
    }
    skips
}

fn shortest_route(
    walls: &FxHashSet<Coordinate>,
    start: Coordinate,
    end: Coordinate,
) -> Option<usize> {
    let mut queue: BinaryHeap<(Reverse<usize>, Coordinate)> =
        BinaryHeap::from_iter([(Reverse(0), start)]);
    let mut visited: FxHashSet<Coordinate> = FxHashSet::from_iter([start]);

    while let Some((Reverse(time), position)) = queue.pop() {
        let options = [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .map(Coordinate::from)
            .map(|d| (d, walls.contains(&(position + d))));
        for (direction, is_wall) in options {
            let new_position = position + direction;
            if new_position == end {
                return Some(time + 1);
            }
            if is_wall {
                continue;
            }
            if !visited.insert(position + direction) {
                continue;
            }
            queue.push((Reverse(time + 1), position + direction));
        }
    }
    None
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
        assert_eq!(part_1(INPUT), 84);
    }
}
