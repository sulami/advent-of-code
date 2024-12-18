use nom::character::complete::{char, u64};
use nom::sequence::separated_pair;
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

super::solve!("18");

fn parse(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|l| {
            separated_pair(u64::<&str, ()>, char(','), u64)(l)
                .expect("invalid pair")
                .1
        })
        .map(|(a, b)| (a as usize, b as usize).into())
        .collect()
}

fn part_1(input: &[Coordinate]) -> usize {
    let test_mode = input.len() < 1024;
    let size = if test_mode { 7 } else { 71 };
    let mut corruption = [[false; 71]; 71];
    input
        .iter()
        .take(if test_mode { 12 } else { 1024 })
        .for_each(|&Coordinate { x, y }| corruption[x][y] = true);
    steps_required(&corruption, size).expect("no solution found")
}

fn part_2(input: &[Coordinate]) -> String {
    let test_mode = input.len() < 1024;
    let size = if test_mode { 7 } else { 71 };
    let indices: Vec<usize> = (0..=input.len()).collect();
    let time = indices.partition_point(|n| {
        let mut corruption = [[false; 71]; 71];
        input
            .iter()
            .take(*n)
            .for_each(|&Coordinate { x, y }| corruption[x][y] = true);
        steps_required(&corruption, size).is_some()
    });
    let fatal_byte = input[time - 1];
    format!("{},{}", fatal_byte.x, fatal_byte.y)
}

fn steps_required(corruption: &[[bool; 71]; 71], size: usize) -> Option<usize> {
    let mut visited = [[false; 71]; 71];
    visited[0][0] = true;

    let mut queue = BinaryHeap::from_iter([Reverse((0, Coordinate::default()))]);
    while let Some(Reverse((steps, coord))) = queue.pop() {
        for neighbour in coord.neighbours(size) {
            if neighbour == (size - 1, size - 1).into() {
                return Some(steps + 1);
            }
            if visited[neighbour.x][neighbour.y] || corruption[neighbour.x][neighbour.y] {
                continue;
            }
            visited[neighbour.x][neighbour.y] = true;
            queue.push(Reverse((steps + 1, neighbour)));
        }
    }

    None
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Default, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coordinate {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x + self.y).cmp(&(other.x + other.y)).reverse()
    }
}

impl Coordinate {
    fn neighbours(&self, size: usize) -> Vec<Self> {
        let mut rv = vec![];
        if self.x > 0 {
            rv.push((self.x - 1, self.y).into());
        }
        if self.x < size - 1 {
            rv.push((self.x + 1, self.y).into());
        }
        if self.y > 0 {
            rv.push((self.x, self.y - 1).into());
        }
        if self.y < size - 1 {
            rv.push((self.x, self.y + 1).into());
        }
        rv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(INPUT)), 22);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&parse(INPUT)), "6,1");
    }
}
