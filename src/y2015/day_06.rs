use crate::coordinate::Coordinate;
use crate::print_results;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, i64};
use nom::combinator::{map, value};
use nom::sequence::separated_pair;
use nom::IResult;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/06");
    let instructions = parse_instructions(input);
    print_results(
        2015,
        6,
        part_1(&instructions),
        part_2(&instructions),
        Some(start),
    );
}

fn part_1(instructions: &[Instruction]) -> usize {
    instructions
        .iter()
        .fold(
            [false; 1000 * 1000],
            |mut state, Instruction { from, to, op }| {
                let change = match op {
                    Op::On => |_| true,
                    Op::Off => |_| false,
                    Op::Toggle => |val| val ^ true,
                };
                for x in from.x..=to.x {
                    for y in from.y..=to.y {
                        state[1000 * x as usize + y as usize] =
                            change(state[1000 * x as usize + y as usize]);
                    }
                }
                state
            },
        )
        .iter()
        .filter(|x| **x)
        .count()
}

fn part_2(instructions: &[Instruction]) -> u32 {
    instructions
        .iter()
        .fold(
            [0_u32; 1000 * 1000],
            |mut state, Instruction { from, to, op }| {
                let change = match op {
                    Op::On => |val| val + 1,
                    Op::Off => |val: u32| val.saturating_sub(1),
                    Op::Toggle => |val| val + 2,
                };
                for x in from.x..=to.x {
                    for y in from.y..=to.y {
                        state[1000 * x as usize + y as usize] =
                            change(state[1000 * x as usize + y as usize]);
                    }
                }
                state
            },
        )
        .iter()
        .sum::<u32>()
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.lines()
        .map(|line| -> IResult<&str, Instruction> {
            let (line, op) = alt((
                value(Op::On, tag("turn on ")),
                value(Op::Off, tag("turn off ")),
                value(Op::Toggle, tag("toggle ")),
            ))(line)?;
            let (line, from) = map(separated_pair(i64, char(','), i64), |(x, y)| {
                Coordinate::new(x as isize, y as isize)
            })(line)?;
            let (line, _) = tag(" through ")(line)?;
            let (line, to) = map(separated_pair(i64, char(','), i64), |(x, y)| {
                Coordinate::new(x as isize, y as isize)
            })(line)?;
            Ok((line, Instruction { op, from, to }))
        })
        .map_ok(|(_, i)| i)
        .collect::<Result<Vec<_>, _>>()
        .expect("invalid input")
}

#[derive(Copy, Clone)]
struct Instruction {
    from: Coordinate,
    to: Coordinate,
    op: Op,
}

#[derive(Copy, Clone)]
enum Op {
    On,
    Off,
    Toggle,
}
