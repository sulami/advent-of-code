#![cfg_attr(not(feature = "day-02"), allow(dead_code))]

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{one_of, u32},
    combinator::{opt, value},
    IResult,
};

pub fn solve() -> (u64, u64) {
    let input = include_str!("../inputs/day_02");

    let mut sum: u32 = 0;
    let mut sum2: u32 = 0;

    input.lines().for_each(|line| {
        let record = GameRecord::from_line(line);
        if record.is_possible() {
            sum += record.id;
        }
        sum2 += record.max_red * record.max_green * record.max_blue;
    });

    (sum as u64, sum2 as u64)
}

#[derive(Default)]
struct GameRecord {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

impl GameRecord {
    fn from_line(s: &str) -> Self {
        parse_record(s).unwrap().1
    }

    fn is_possible(&self) -> bool {
        self.max_red <= 12 && self.max_green <= 13 && self.max_blue <= 14
    }
}

fn parse_record(s: &str) -> IResult<&str, GameRecord> {
    let (mut max_red, mut max_green, mut max_blue) = (0, 0, 0);

    let (s, _) = tag("Game ")(s)?;
    let (s, id) = u32(s)?;
    let (mut s, _) = tag(": ")(s)?;

    loop {
        if s.is_empty() {
            break;
        }
        let (rest, cubes) = parse_cubes(s)?;
        match cubes {
            Cubes::Red(n) => {
                max_red = max_red.max(n);
            }
            Cubes::Green(n) => {
                max_green = max_green.max(n);
            }
            Cubes::Blue(n) => {
                max_blue = max_blue.max(n);
            }
        }
        let (rest, _) = opt(one_of(",;"))(rest)?;
        let (rest, _) = opt(tag(" "))(rest)?;
        s = rest;
    }

    Ok((
        s,
        GameRecord {
            id,
            max_red,
            max_green,
            max_blue,
        },
    ))
}

#[derive(Copy, Clone)]
enum Cubes {
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse_cubes(s: &str) -> IResult<&str, Cubes> {
    let (s, count) = u32(s)?;
    let (s, _) = tag(" ")(s)?;
    alt((
        value(Cubes::Red(count), tag("red")),
        value(Cubes::Green(count), tag("green")),
        value(Cubes::Blue(count), tag("blue")),
    ))(s)
}
