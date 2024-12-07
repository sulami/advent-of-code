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

#[derive(Clone, Default)]
struct Formula {
    target: u64,
    elements: Vec<u64>,
}

#[derive(Copy, Clone)]
enum Operator {
    Plus,
    Times,
    Concat,
}

impl Formula {
    fn is_possible(&self, with_concat: bool) -> bool {
        self.search(with_concat, &mut vec![])
    }

    fn search(&self, with_concat: bool, ops: &mut Vec<Operator>) -> bool {
        let result = self.apply_operators(ops);
        if result > self.target {
            false
        } else if ops.len() + 1 == self.elements.len() {
            result == self.target
        } else {
            if with_concat {
                ops.push(Operator::Concat);
                if self.search(with_concat, ops) {
                    return true;
                }
                ops.pop();
            }
            ops.push(Operator::Times);
            if self.search(with_concat, ops) {
                return true;
            }
            ops.pop();
            ops.push(Operator::Plus);
            if self.search(with_concat, ops) {
                return true;
            }
            ops.pop();
            false
        }
    }

    fn apply_operators(&self, operators: &[Operator]) -> u64 {
        self.elements.iter().skip(1).zip(operators.iter()).fold(
            *self.elements.first().unwrap_or(&0),
            |acc, (next, op)| match op {
                Operator::Plus => acc + next,
                Operator::Times => acc * next,
                Operator::Concat => acc * 10_u64.pow(next.checked_ilog10().unwrap_or(0) + 1) + next,
            },
        )
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
