use itertools::Itertools;
use nom::{
    character::complete::{i32 as parse_i32, multispace1},
    combinator::all_consuming,
    multi::separated_list1,
    IResult,
};

super::solve!("02");

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|l| parse_report(l).expect("invalid report").1)
        .filter(|r| safe_report(r))
        .count()
}

fn part_2(input: &str) -> usize {
    let mut count = 0;
    for report in input
        .lines()
        .map(|l| parse_report(l).expect("invalid report").1)
    {
        if safe_report(&report) {
            count += 1;
        } else {
            for i in 0..report.len() {
                if safe_report(
                    &report
                        .iter()
                        .copied()
                        .take(i)
                        .chain(report.iter().copied().skip(i + 1))
                        .collect::<Vec<_>>(),
                ) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

fn safe_report(report: &[i32]) -> bool {
    let mut signum = 0;
    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        if !(1..=3).contains(&diff.abs()) {
            return false;
        }

        if signum == 0 {
            signum = diff.signum();
        } else if signum != diff.signum() {
            return false;
        }
    }
    true
}

fn parse_report(s: &str) -> IResult<&str, Vec<i32>> {
    all_consuming(separated_list1(multispace1, parse_i32))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 4);
    }
}
