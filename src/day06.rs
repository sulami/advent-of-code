use itertools::Itertools;
use std::collections::HashSet;

super::solve!("06");

fn part_1(input: &str) -> usize {
    Map::from_str(input).walk().len()
}

fn part_2(input: &str) -> usize {
    let mut count = 0;
    let mut map = Map::from_str(input);
    let initial_position = map.position;
    let initial_direction = map.direction;

    let normally_visited = map.walk();
    map.position = initial_position;
    map.direction = initial_direction;

    let spaces = map
        .inner
        .iter()
        .enumerate()
        .positions(|(idx, c)| *c == '.' && normally_visited.contains(&idx))
        .collect_vec();

    for idx in spaces {
        map.inner[idx] = '#';
        if map.will_loop() {
            count += 1;
        }

        // Reset the map state.
        map.inner[idx] = '.';
        map.position = initial_position;
        map.direction = initial_direction;
    }
    count
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
struct Map {
    width: usize,
    inner: Vec<char>,
    position: usize,
    direction: Direction,
}

impl Map {
    fn from_str(s: &str) -> Self {
        let width = s.lines().next().expect("no lines").len();
        let inner = s.chars().filter(|&c| ".#^".contains(c)).collect_vec();
        let position = inner
            .iter()
            .position(|&c| c == '^')
            .expect("start not found");
        Self {
            width,
            inner,
            position,
            direction: Direction::Up,
        }
    }

    /// Walk until leaving or looping, returning whether looped.
    fn will_loop(&mut self) -> bool {
        let mut locations = HashSet::from([(self.position, self.direction)]);
        while self.step() {
            if locations.contains(&(self.position, self.direction)) {
                return true;
            }
            locations.insert((self.position, self.direction));
        }
        false
    }

    /// Walk until leaving map, returning all positions visited.
    fn walk(&mut self) -> HashSet<usize> {
        let mut locations = HashSet::from([self.position]);
        while self.step() {
            locations.insert(self.position);
        }
        locations
    }

    fn step(&mut self) -> bool {
        // Are we leaving as is?
        if self.is_leaving() {
            return false;
        }
        // If facing an obstacle, turn.
        if self.is_facing_obstacle() {
            self.turn();
        }
        // Are we leaving after turning?
        if self.is_leaving() {
            return false;
        }
        // If we aren't facing an obstacle after turning once, walk. Otherwise, leave it at turning,
        // so we can record the new direction in loop detection.
        if !self.is_facing_obstacle() {
            self.position = self.next_position();
        }
        true
    }

    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Down => Direction::Left,
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    /// Not guaranteed to be in bounds/non-wrapping.
    fn next_position(&self) -> usize {
        match self.direction {
            Direction::Down => self.position + self.width,
            Direction::Up => self.position - self.width,
            Direction::Left => self.position - 1,
            Direction::Right => self.position + 1,
        }
    }

    fn is_leaving(&self) -> bool {
        match self.direction {
            Direction::Right => self.position % self.width == self.width - 1,
            Direction::Left => self.position % self.width == 0,
            Direction::Up => self.position < self.width,
            Direction::Down => self.position >= self.inner.len() - self.width,
        }
    }

    /// Assumes we're not about to walk off the map, otherwise will show phantom obstacles due to
    /// wrapping.
    fn is_facing_obstacle(&self) -> bool {
        self.inner
            .get(self.next_position())
            .map(|&c| c == '#')
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(INPUT), 41);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(INPUT), 6);
    }
}