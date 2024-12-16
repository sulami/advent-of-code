use rustc_hash::FxHashSet;
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

    search(walls, start, Direction::East, finish).1.len()
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

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Action {
    Move,
    TurnClockwise,
    TurnCounterClockwise,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Possibility {
    position: Coords,
    direction: Direction,
    finish: Coords,
    score: usize,
    action: Action,
    visited: FxHashSet<(Coords, Direction)>,
}

impl PartialOrd for Possibility {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Possibility {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            .reverse()
            .then_with(|| {
                manhattan_distance(self.position, self.finish)
                    .cmp(&manhattan_distance(other.position, other.finish))
                    .reverse()
            })
            .then_with(|| {
                manhattan_distance(next_position(self.position, self.direction), self.finish)
                    .cmp(&manhattan_distance(
                        next_position(other.position, other.direction),
                        other.finish,
                    ))
                    .reverse()
            })
    }
}

fn search(
    walls: &FxHashSet<Coords>,
    start: Coords,
    direction: Direction,
    finish: Coords,
) -> (usize, FxHashSet<Coords>) {
    let mut options = BinaryHeap::new();
    let mut costs = [[None; 141]; 141];
    costs[start.0 as usize][start.1 as usize] = Some(0);

    for action in [
        Action::Move,
        Action::TurnClockwise,
        Action::TurnCounterClockwise,
    ] {
        options.push(Possibility {
            position: start,
            direction,
            finish,
            score: 0,
            action,
            visited: FxHashSet::from_iter([(start, direction)]),
        });
    }

    let mut high_score = None;
    let mut all_benches = FxHashSet::default();

    while let Some(Possibility {
        position,
        direction,
        score,
        action,
        visited,
        ..
    }) = options.pop()
    {
        let mut push_new_possibilities =
            |position, direction, score, visited: &FxHashSet<(Coords, Direction)>| {
                if visited.contains(&(position, direction)) {
                    return;
                }
                for action in [
                    Action::Move,
                    Action::TurnClockwise,
                    Action::TurnCounterClockwise,
                ] {
                    let mut visited = visited.to_owned();
                    visited.insert((position, direction));
                    options.push(Possibility {
                        position,
                        direction,
                        finish,
                        score,
                        action,
                        visited,
                    });
                }
            };

        // Fill out the minimum scores to get to a place.
        match costs[position.0 as usize][position.1 as usize] {
            // If we can't get here and turn around twice for the current score, this is suboptimal.
            Some(c) if c + 2_000 < score => continue,
            Some(c) if score < c => costs[position.0 as usize][position.1 as usize] = Some(score),
            Some(_) => {}
            None => costs[position.0 as usize][position.1 as usize] = Some(score),
        }

        // We're done here.
        if position == finish {
            match high_score {
                Some(hs) if hs == score => {
                    all_benches.extend(visited.into_iter());
                }
                Some(_) => {}
                None => {
                    high_score = Some(score);
                    all_benches.extend(visited.into_iter());
                }
            }
            continue;
        }

        // Add more possibilities based on the current one.
        match action {
            Action::Move => {
                if !walls.contains(&next_position(position, direction)) {
                    push_new_possibilities(
                        next_position(position, direction),
                        direction,
                        score + 1,
                        &visited,
                    );
                }
            }
            Action::TurnClockwise => {
                push_new_possibilities(
                    position,
                    direction.turn_clockwise(),
                    score + 1000,
                    &visited,
                );
            }
            Action::TurnCounterClockwise => {
                push_new_possibilities(
                    position,
                    direction.turn_counterclockwise(),
                    score + 1000,
                    &visited,
                );
            }
        }
    }

    (
        high_score.expect("no solution found"),
        all_benches.iter().map(|(p, _)| *p).collect(),
    )
}

fn next_position((x, y): Coords, direction: Direction) -> Coords {
    match direction {
        Direction::East => (x + 1, y),
        Direction::West => (x - 1, y),
        Direction::North => (x, y - 1),
        Direction::South => (x, y + 1),
    }
}

fn manhattan_distance((ax, ay): Coords, (bx, by): Coords) -> usize {
    ax.abs_diff(bx) + ay.abs_diff(by)
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
}
