use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, u32 as parse_u32},
    combinator::map,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
};

crate::solve!("03");

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse(input: &str) -> Vec<Instruction> {
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
    many1(map(many_till(anychar::<&str, ()>, instruction), |(_, x)| x))(input)
        .expect("failed to parse input")
        .1
}

fn part_1(instructions: &[Instruction]) -> u32 {
    instructions
        .iter()
        .filter_map(|instruction| match instruction {
            Instruction::Mul(a, b) => Some(a * b),
            _ => None,
        })
        .sum()
}

fn part_2(instructions: &[Instruction]) -> u32 {
    let mut active = true;
    let mut sum = 0;
    for instruction in instructions {
        match instruction {
            Instruction::Mul(a, b) if active => sum += a * b,
            Instruction::Mul(_, _) => {}
            Instruction::Do => active = true,
            Instruction::Dont => active = false,
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(&parse(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            )),
            161
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(&parse(
                "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
            )),
            48
        );
    }
}
