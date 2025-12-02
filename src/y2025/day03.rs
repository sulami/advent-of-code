crate::solve!("03");

type Parsed = Vec<Vec<char>>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn part_1(x: &Parsed) -> u128 {
    x.iter()
        .map(|bank| {
            let (tens, tens_idx) = find_best_digit(&bank[..bank.len() - 1]);
            let (ones, _) = find_best_digit(&bank[tens_idx + 1..]);
            tens * 10 + ones
        })
        .sum()
}

fn part_2(x: &Parsed) -> u128 {
    x.iter()
        .map(|bank| {
            let mut start = 0;
            let mut sum = 0;
            for digit in 0..12 {
                let (d, idx) = find_best_digit(&bank[start..bank.len() - (11 - digit)]);
                start += idx + 1;
                sum *= 10;
                sum += d;
            }
            sum
        })
        .sum()
}

/// Finds the largest digit apart from the last one, returning it and its index.
/// If there are several of the largest digit, takes the first one to keep as many options as possible.
fn find_best_digit(bank: &[char]) -> (u128, usize) {
    let best_value = bank.iter().max().unwrap();
    let best_idx = bank.iter().position(|c| c == best_value).unwrap();
    (best_value.to_digit(10).unwrap() as u128, best_idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(&vec![
                987654321111111u128.to_string().chars().collect(),
                811111111111119u128.to_string().chars().collect(),
                234234234234278u128.to_string().chars().collect(),
                818181911112111u128.to_string().chars().collect(),
            ]),
            357
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(&vec![
                987654321111111u128.to_string().chars().collect(),
                811111111111119u128.to_string().chars().collect(),
                234234234234278u128.to_string().chars().collect(),
                818181911112111u128.to_string().chars().collect(),
            ]),
            3121910778619
        );
    }
}
