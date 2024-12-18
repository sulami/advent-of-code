use rustc_hash::FxHashSet;
use std::{cmp::Ordering, collections::BinaryHeap, hash::Hash};

crate::solve!("16");

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
    let mut costs = [[[None; 4]; 141]; 141];
    costs[start.0 as usize][start.1 as usize][direction as usize] = Some(0);
    let mut options = BinaryHeap::from([Possibility {
        position: start,
        direction,
        finish,
        score: 0,
    }]);

    while let Some(Possibility {
        position,
        direction,
        score,
        ..
    }) = options.pop()
    {
        // Add more possibilities based on the current one.
        for (new_position, new_direction, new_score) in [
            (next_position(position, direction), direction, score + 1),
            (position, direction.turn_clockwise(), score + 1000),
            (position, direction.turn_counterclockwise(), score + 1000),
        ] {
            // Don't walk into walls.
            if walls.contains(&new_position) {
                continue;
            }

            let cost = &mut costs[new_position.0 as usize][new_position.1 as usize];

            match cost[new_direction as usize] {
                // We got here with a lower score before, abort.
                Some(c) if c < new_score => continue,
                // We got here with a lower score than before, update the score.
                Some(c) if new_score < c => cost[new_direction as usize] = Some(new_score),
                // We got here before with the same score, found an alternative route.
                Some(_) => {}
                // We haven't been here before, fill in the score.
                None => cost[new_direction as usize] = Some(new_score),
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

    let min_cost = costs[finish.0 as usize][finish.1 as usize]
        .into_iter()
        .min()
        .flatten()
        .expect("no solution found");
    let fields = min_path_fields(walls, &costs, start, finish);

    (min_cost, fields.len())
}

fn min_path_fields(
    walls: &FxHashSet<Coords>,
    costs: &[[[Option<usize>; 4]; 141]; 141],
    start: Coords,
    finish: Coords,
) -> FxHashSet<Coords> {
    let mut cost_to_visit = [[usize::MAX; 141]; 141];
    let mut benches = [[[false; 4]; 141]; 141];
    let mut queue = BinaryHeap::from([Possibility {
        position: start,
        direction: Direction::East,
        score: costs[start.0 as usize][start.1 as usize][Direction::East as usize].unwrap(),
        finish,
    }]);

    while let Some(Possibility {
        position,
        direction,
        score,
        ..
    }) = queue.pop()
    {
        let (x, y) = (position.0 as usize, position.1 as usize);

        if position == finish {
            benches[x][y][direction as usize] = true;
            break;
        }

        let (turns, steps) = (score / 1000, score % 1000);
        let min = cost_to_visit[x][y];
        let (min_turns, min_steps) = (min / 1000, min % 1000);
        if min_turns + 1 < turns {
            continue;
        }
        if min_steps < steps {
            continue;
        }
        if steps < min_steps {
            benches[x][y].fill(false);
        }

        cost_to_visit[x][y] = score;
        benches[x][y][direction as usize] = true;

        if !walls.contains(&next_position(position, direction)) {
            queue.push(Possibility {
                position: next_position(position, direction),
                direction,
                score: score + 1,
                finish,
            })
        }

        if !walls.contains(&next_position(position, direction.turn_clockwise())) {
            queue.push(Possibility {
                position: next_position(position, direction.turn_clockwise()),
                direction: direction.turn_clockwise(),
                score: score + 1001,
                finish,
            })
        }

        if !walls.contains(&next_position(position, direction.turn_counterclockwise())) {
            queue.push(Possibility {
                position: next_position(position, direction.turn_counterclockwise()),
                direction: direction.turn_counterclockwise(),
                score: score + 1001,
                finish,
            })
        }
    }

    let mut rv = FxHashSet::from_iter([start]);
    let mut to_add = vec![finish];
    while let Some(p @ (x, y)) = to_add.pop() {
        if !rv.insert(p) {
            continue;
        }
        let directions = benches[x as usize][y as usize];
        if directions[Direction::East as usize] {
            to_add.push(next_position(p, Direction::West));
        }
        if directions[Direction::West as usize] {
            to_add.push(next_position(p, Direction::East));
        }
        if directions[Direction::North as usize] {
            to_add.push(next_position(p, Direction::South));
        }
        if directions[Direction::South as usize] {
            to_add.push(next_position(p, Direction::North));
        }
    }
    rv
}

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
