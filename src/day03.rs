use nom::branch::alt;
use nom::multi::many1;
use nom::{
    bytes::complete::tag,
    character::complete::u32 as parse_u32,
    character::complete::{anychar, char},
    combinator::map,
    multi::many_till,
    sequence::{delimited, separated_pair},
    IResult,
};

pub fn solve() {
    let input = include_str!("inputs/03");
    println!("{}", part_1(input));
    println!("{}", part_2(input));
}

fn part_1(input: &str) -> u32 {
    parse_memory(input)
        .expect("invalid memory")
        .1
        .iter()
        .filter_map(|instruction| match instruction {
            Instruction::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    let mut active = true;
    let mut sum = 0;
    for instruction in parse_memory(input).expect("invalid memory").1 {
        match instruction {
            Instruction::Mul(a, b) if active => sum += a * b,
            Instruction::Mul(_, _) => {}
            Instruction::Do => active = true,
            Instruction::Dont => active = false,
        }
    }
    sum
}

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_memory(s: &str) -> IResult<&str, Vec<Instruction>> {
    let parse_mul = map(
        delimited(
            tag("mul("),
            separated_pair(parse_u32, char(','), parse_u32),
            char(')'),
        ),
        |(a, b)| Instruction::Mul(a, b),
    );
    let parse_do = map(tag("do()"), |_| Instruction::Do);
    let parse_dont = map(tag("don't()"), |_| Instruction::Dont);
    let instruction = alt((parse_mul, parse_do, parse_dont));
    many1(map(many_till(anychar, instruction), |(_, x)| x))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"),
            161
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"),
            48
        );
    }
}
