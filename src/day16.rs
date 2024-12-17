use rustc_hash::{FxHashMap, FxHashSet};
use std::{cmp::Ordering, collections::BinaryHeap, hash::Hash};

super::solve!("16");

fn parse(input: &str) -> (FxHashSet<Coords>, Coords, Coords) {
    let mut walls = FxHashSet::default();
    let mut start = (0, 0);
    let mut finish = (0, 0);
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((x as isize, y as isize));
                }
                'S' => {
                    start = (x as isize, y as isize);
                }
                'E' => {
                    finish = (x as isize, y as isize);
                }
                _ => {}
            }
        }
    }

    (walls, start, finish)
}

fn part_1((walls, start, finish): &(FxHashSet<Coords>, Coords, Coords)) -> usize {
    let start = start.to_owned();
    let finish = finish.to_owned();

    search(walls, start, Direction::East, finish).0
}

fn part_2((walls, start, finish): &(FxHashSet<Coords>, Coords, Coords)) -> usize {
    let start = start.to_owned();
    let finish = finish.to_owned();

    search(walls, start, Direction::East, finish).1
}

type Coords = (isize, isize);

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn turn_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn turn_counterclockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Possibility {
    position: Coords,
    direction: Direction,
    finish: Coords,
    score: usize,
}

impl PartialOrd for Possibility {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Possibility {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

fn search(
    walls: &FxHashSet<Coords>,
    start: Coords,
    direction: Direction,
    finish: Coords,
) -> (usize, usize) {
    let mut parents: FxHashMap<Coords, Vec<Coords>> = FxHashMap::default();
    let mut costs = [[[None; 4]; 141]; 141];
    costs[start.1 as usize][start.0 as usize][direction as usize] = Some(0);
    let mut options = BinaryHeap::new();
    options.push(Possibility {
        position: start,
        direction,
        finish,
        score: 0,
    });

    while let Some(Possibility {
        position,
        direction,
        score,
        ..
    }) = options.pop()
    {
        // Add more possibilities based on the current one. Conflate turning and moving, because
        // we never want to do a 180 anyway.
        'step: for (new_position, new_direction, new_score) in [
            (next_position(position, direction), direction, score + 1),
            (
                next_position(position, direction.turn_clockwise()),
                direction.turn_clockwise(),
                score + 1001,
            ),
            (
                next_position(position, direction.turn_counterclockwise()),
                direction.turn_counterclockwise(),
                score + 1001,
            ),
        ] {
            // Don't walk into walls.
            if walls.contains(&new_position) {
                continue;
            }

            let cost = &mut costs[new_position.1 as usize][new_position.0 as usize];

            // Special case for the finish, we don't care about direction, only the cheapest way to
            // get to the position in any direction.
            if new_position == finish {
                match cost[0] {
                    // We haven't been to the finish before.
                    None => {
                        cost[0] = Some(new_score);
                        parents.insert(new_position, vec![position]);
                    }
                    // We got to the finish with a lower score than before.
                    Some(s) if new_score < s => {
                        cost[0] = Some(new_score);
                        parents.insert(new_position, vec![position]);
                    }
                    // We got to the finish with the same score as before.
                    Some(s) if new_score == s => {
                        parents
                            .entry(new_position)
                            .and_modify(|v| {
                                if !v.contains(&position) {
                                    v.push(position)
                                }
                            })
                            .or_insert(vec![position]);
                    }
                    // We got to the finish with a higher score than before.
                    Some(s) if s < new_score => {}
                    _ => unreachable!("s and score compare weirdly"),
                }
                // We're done here.
                continue;
            }

            // Check turns on either side of the current direction.
            for turn in [
                (new_direction as usize + 1) % 4,
                (new_direction as usize + 3) % 4,
            ] {
                // We got here before from a different direction, but at lower cost.
                if cost[turn].is_some_and(|c| c + 1000 < new_score) {
                    continue 'step;
                }
                // We got here before from a different direction, but at a higher cost.
                if cost[turn].is_some_and(|c| c + 1000 > new_score) {
                    parents.remove(&new_position);
                }
            }

            // We got here from the opposite side cheaper.
            if cost[(new_direction as usize + 2) % 4].is_some_and(|c| c + 2000 < new_score) {
                continue;
            }

            // Check the same direction we arrived.
            match cost[new_direction as usize] {
                // We got here with a lower score before, abort.
                Some(c) if c < new_score => continue,
                // We got here with a lower score than before, update the score.
                Some(c) if new_score < c => {
                    cost[new_direction as usize] = Some(new_score);
                    // Fill in cost for the two turns as well.
                    for turn in [
                        (new_direction as usize + 1) % 4,
                        (new_direction as usize + 3) % 4,
                    ] {
                        cost[turn] = Some(new_score + 1000);
                    }
                    // Previous ways to get here were suboptimal, reset parents.
                    parents.remove(&new_position);
                }
                // We got here before with the same score, found an alternative route.
                Some(_) => {}
                // We haven't been here before, fill in the score and route.
                None => {
                    cost[new_direction as usize] = Some(new_score);
                }
            }

            // Record which position we got here from.
            match parents.get_mut(&new_position) {
                Some(v) if !v.contains(&position) => v.push(position),
                None => {
                    parents.insert(new_position, vec![position]);
                }
                Some(_) => {}
            }

            // This is a valid next step, continue from there later.
            options.push(Possibility {
                position: new_position,
                direction: new_direction,
                score: new_score,
                finish,
            });
        }
    }

    let mut benches: FxHashSet<Coords> = FxHashSet::default();
    let mut current = vec![finish];
    while let Some(p) = current.pop() {
        benches.insert(p);
        current.extend(
            parents
                .get(&p)
                .unwrap_or(&vec![])
                .iter()
                .filter(|c| !benches.contains(c)),
        );
    }
    (
        costs[finish.1 as usize][finish.0 as usize][0].expect("no solution found"),
        benches.len(),
    )
}
// use itertools::Itertools;
// println!(
//     "{:?}",
//     benches
//         .iter()
//         .sorted_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)))
//         .collect_vec()
// );
// for (k, v) in parents
//     .iter()
//     .sorted_by(|(a, _), (b, _)| a.0.cmp(&b.0).then(a.1.cmp(&b.1)))
// {
//     println!("{k:?}: {v:?}");
// }
// for line in costs {
//     for entry in line {
//         print!(
//             "{:>5}",
//             entry
//                 .iter()
//                 .copied()
//                 .reduce(|a, b| a.min(b))
//                 .flatten()
//                 .unwrap_or(0)
//         );
//     }
//     println!();
// }

fn next_position((x, y): Coords, direction: Direction) -> Coords {
    match direction {
        Direction::East => (x + 1, y),
        Direction::West => (x - 1, y),
        Direction::North => (x, y - 1),
        Direction::South => (x, y + 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(INPUT)), 7036);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&parse(INPUT)), 45);
    }

    #[test]
    fn test_part2_alternative() {
        let input = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(part_2(&parse(input)), 64);
    }
}
