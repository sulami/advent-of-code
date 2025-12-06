use itertools::Itertools;

crate::solve!("06");

type Parsed = Vec<(Vec<String>, char)>;

fn parse(input: &str) -> Parsed {
    let mut problems: Vec<(Vec<String>, char, usize)> = vec![];

    for (idx, c) in input
        .lines()
        .last()
        .unwrap()
        .chars()
        .enumerate()
        .filter(|(_, c)| !c.is_ascii_whitespace())
    {
        problems.push((vec![], c, idx));
    }

    for line in input.lines() {
        if line.chars().any(|c| "+*".contains(c)) {
            break;
        }

        for (numbers, _, start) in problems.iter_mut() {
            let mut seen_number = false;
            numbers.push(
                line.chars()
                    .skip(*start)
                    .take_while(|c| {
                        if c.is_ascii_digit() {
                            seen_number = true;
                            true
                        } else {
                            !seen_number
                        }
                    })
                    .collect(),
            );
        }
    }

    problems
        .into_iter()
        .map(|(x, y, _)| (x, y))
        .map(|(mut xs, op)| {
            let len = xs.iter().map(|s| s.len()).max().unwrap();
            xs.iter_mut().for_each(|s| {
                (0..len - s.len()).for_each(|_| s.push(' '));
            });
            (xs, op)
        })
        .collect()
}

fn part_1(x: &Parsed) -> usize {
    x.iter().map(|(nums, op)| maths(&nums, *op)).sum()
}

fn part_2(x: &Parsed) -> usize {
    x.iter()
        .map(|(nums, op)| (transpose(nums.clone()), op))
        .map(|(nums, op)| maths(&nums, *op))
        .sum()
}

fn maths(numbers: &[String], op: char) -> usize {
    let iter = numbers
        .iter()
        .map(|s| s.trim_ascii().parse::<usize>().unwrap());
    match op {
        '+' => iter.sum(),
        '*' => iter.product(),
        _ => unreachable!(),
    }
}

fn transpose(xs: Vec<String>) -> Vec<String> {
    let chars = xs.iter().map(|s| s.chars().collect_vec()).collect_vec();
    let height = chars.len();
    let width = chars[0].len();

    (0..width)
        .map(|x| {
            (0..height).fold(String::new(), |mut s, y| {
                s.push(chars[y][x]);
                s
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   + ";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse(INPUT)), 4277556);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(INPUT)), 3263827);
    }
}
