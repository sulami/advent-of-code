use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, i32 as parse_i32, space1},
    sequence::{preceded, separated_pair},
    IResult,
};

super::solve!("14");

fn parse(input: &str) -> Vec<Robot> {
    let parse_robot = |s| -> IResult<&str, Robot> {
        let (s, position) =
            preceded(tag("p="), separated_pair(parse_i32, char(','), parse_i32))(s)?;
        let (s, _) = space1(s)?;
        let (s, velocity) =
            preceded(tag("v="), separated_pair(parse_i32, char(','), parse_i32))(s)?;
        Ok((
            s,
            Robot {
                position: XY {
                    x: position.0,
                    y: position.1,
                },
                velocity: XY {
                    x: velocity.0,
                    y: velocity.1,
                },
            },
        ))
    };
    input
        .lines()
        .map(|l| parse_robot(l).expect("invalid robot").1)
        .collect()
}

fn part_1(robots: &[Robot]) -> usize {
    let size = if robots.len() > 100 {
        (101, 103)
    } else {
        (11, 7)
    };
    let mut robots = robots.to_vec();
    (0..100).for_each(|_| robots.iter_mut().for_each(|b| b.step(size)));
    safety_factor(&robots, size)
}

fn part_2(robots: &[Robot]) -> usize {
    let size = if robots.len() > 10 {
        (101, 103)
    } else {
        (11, 7)
    };
    let mut robots = robots.to_vec();
    let mut steps = 0;
    while !has_line(&robots) {
        steps += 1;
        robots.iter_mut().for_each(|b| b.step(size));
    }
    steps
}

#[derive(Copy, Clone, Debug)]
struct XY {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug)]
struct Robot {
    position: XY,
    velocity: XY,
}

impl Robot {
    fn step(&mut self, (width, height): (usize, usize)) {
        self.position.x = (width as i32 + self.position.x + self.velocity.x) % width as i32;
        self.position.y = (height as i32 + self.position.y + self.velocity.y) % height as i32;
    }
}

fn safety_factor(robots: &[Robot], (width, height): (usize, usize)) -> usize {
    let quadrants = robots.iter().into_group_map_by(|b| {
        match (
            ((width as i32) / 2 - b.position.x).signum(),
            ((height as i32) / 2 - b.position.y).signum(),
        ) {
            (-1, -1) => 1,
            (-1, 1) => 2,
            (1, -1) => 3,
            (1, 1) => 4,
            _ => 0,
        }
    });
    (1..=4)
        .map(|q| quadrants.get(&q).unwrap_or(&vec![]).len())
        .product()
}

fn has_line(robots: &[Robot]) -> bool {
    let mut y_positions = [0; 103];
    for robot in robots {
        y_positions[robot.position.y as usize] += 1;
    }
    let busiest_y = y_positions.iter().position_max().unwrap_or_default() as i32;
    if y_positions[busiest_y as usize] < 20 {
        return false;
    }
    20 < robots
        .iter()
        .filter(|b| b.position.y == busiest_y)
        .map(|b| b.position.x)
        .sorted_unstable()
        .tuple_windows()
        .filter(|(a, b)| a.abs_diff(*b) == 1)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(INPUT)), 12);
    }
}
