#![cfg_attr(not(feature = "day-01"), allow(dead_code))]

use arrayvec::ArrayString;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::one_of,
    combinator::{map, value},
    IResult,
};

pub fn solve() -> (u64, u64) {
    let input = include_str!("../inputs/day_01");

    let mut sum: u32 = 0;
    let mut sum2: u32 = 0;

    input.lines().for_each(|line| {
        // Part 1
        let msd = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let lsd = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
        sum += msd.to_digit(10).unwrap() * 10 + lsd.to_digit(10).unwrap();

        // Part 2
        let msd = find_number(line, false);
        let lsd = find_number(line, true);
        sum2 += msd * 10 + lsd;
    });

    (sum as u64, sum2 as u64)
}

fn find_number(s: &str, reverse: bool) -> u32 {
    let mut idx = 0;
    let matcher = if reverse {
        reverse_number_matcher
    } else {
        number_matcher
    };
    let haystack: ArrayString<64> = if reverse {
        let mut array_s = ArrayString::<64>::new();
        s.chars().rev().for_each(|c| array_s.push(c));
        array_s
    } else {
        ArrayString::from(s).unwrap()
    };

    loop {
        if let Ok((_, n)) = matcher(&haystack[idx..]) {
            return n;
        }
        idx += 1;
    }
}

fn number_matcher(s: &str) -> IResult<&str, u32> {
    alt((
        map(one_of("0123456789"), |c| c.to_digit(10).unwrap()),
        alt((
            value(0, tag("zero")),
            value(1, tag("one")),
            value(2, tag("two")),
            value(3, tag("three")),
            value(4, tag("four")),
            value(5, tag("five")),
            value(6, tag("six")),
            value(7, tag("seven")),
            value(8, tag("eight")),
            value(9, tag("nine")),
        )),
    ))(s)
}

fn reverse_number_matcher(s: &str) -> IResult<&str, u32> {
    alt((
        map(one_of("0123456789"), |c| c.to_digit(10).unwrap()),
        alt((
            value(0, tag("orez")),
            value(1, tag("eno")),
            value(2, tag("owt")),
            value(3, tag("eerht")),
            value(4, tag("ruof")),
            value(5, tag("evif")),
            value(6, tag("xis")),
            value(7, tag("neves")),
            value(8, tag("thgie")),
            value(9, tag("enin")),
        )),
    ))(s)
}
