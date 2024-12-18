crate::solve!("01");

fn parse(input: &str) -> String {
    input.to_string()
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
        assert_eq!(part_1(&parse("(())")), 0);
        assert_eq!(part_1(&parse("()()")), 0);
        assert_eq!(part_1(&parse("(((")), 3);
        assert_eq!(part_1(&parse("(()(()(")), 3);
        assert_eq!(part_1(&parse("))(((((")), 3);
        assert_eq!(part_1(&parse("())")), -1);
        assert_eq!(part_1(&parse("))(")), -1);
        assert_eq!(part_1(&parse(")())())")), -3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(")")), 1);
        assert_eq!(part_2(&parse("()())")), 5);
    }
}
