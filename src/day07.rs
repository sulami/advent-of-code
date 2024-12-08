use nom::bytes::complete::tag;
use nom::character::complete::{newline, space1, u64 as parse_u64};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

super::solve!("07");

fn part_1(input: &str) -> u64 {
    let formulas = parse_input(input).expect("invalid input").1;
    formulas
        .iter()
        .filter(|f| f.is_possible(false))
        .map(|f| f.target)
        .sum()
}

fn part_2(input: &str) -> u64 {
    let formulas = parse_input(input).expect("invalid input").1;
    formulas
        .iter()
        .filter(|f| f.is_possible(true))
        .map(|f| f.target)
        .sum()
}

#[derive(Copy, Clone)]
enum Operator {
    Plus,
    Times,
    Concat,
}

#[derive(Clone)]
struct Formula {
    target: u64,
    elements: Vec<u64>,
}

impl Formula {
    fn is_possible(&self, with_concat: bool) -> bool {
        search(self.target, 0, &self.elements, with_concat)
    }
}

fn parse_input(s: &str) -> IResult<&str, Vec<Formula>> {
    separated_list1(
        newline,
        map(
            separated_pair(parse_u64, tag(": "), separated_list1(space1, parse_u64)),
            |(sum, elements)| Formula {
                target: sum,
                elements,
            },
        ),
    )(s)
}

fn search(target: u64, current: u64, remaining: &[u64], with_concat: bool) -> bool {
    let try_operator = |op| {
        search(
            target,
            apply_operator(current, op, *remaining.first().unwrap()),
            &remaining[1..],
            with_concat,
        )
    };
    if remaining.is_empty() {
        current == target
    } else if current >= target {
        false
    } else {
        (with_concat && try_operator(Operator::Concat))
            || try_operator(Operator::Times)
            || try_operator(Operator::Plus)
    }
}

fn apply_operator(current: u64, operator: Operator, other: u64) -> u64 {
    match operator {
        Operator::Plus => current + other,
        Operator::Times => current.max(1) * other,
        Operator::Concat => current * 10_u64.pow(other.checked_ilog10().unwrap_or(0) + 1) + other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(INPUT), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(INPUT), 11387);
    }
}
