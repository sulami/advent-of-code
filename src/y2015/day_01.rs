use crate::print_results;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/01");
    let pt1 = part_1(input);
    let pt2 = part_2(input);
    print_results(2015, 1, pt1, pt2, Some(start));
}

fn part_1(input: &str) -> i32 {
    input.chars().map(char_to_floor).sum()
}

fn part_2(input: &str) -> usize {
    let mut floor = 0;
    for (idx, c) in input.chars().enumerate() {
        floor += char_to_floor(c);
        if floor < 0 {
            return idx + 1;
        }
    }
    unreachable!("never reached basement")
}

fn char_to_floor(c: char) -> i32 {
    match c {
        '(' => 1,
        ')' => -1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("(())"), 0);
        assert_eq!(part_1("()()"), 0);
        assert_eq!(part_1("((("), 3);
        assert_eq!(part_1("(()(()("), 3);
        assert_eq!(part_1("))((((("), 3);
        assert_eq!(part_1("())"), -1);
        assert_eq!(part_1("))("), -1);
        assert_eq!(part_1(")())())"), -3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(")"), 1);
        assert_eq!(part_2("()())"), 5);
    }
}
