use std::ops::RangeInclusive;

crate::solve!("02");

type Parsed = Vec<RangeInclusive<usize>>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| {
            let (start, end) = s.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect()
}

fn part_1(x: &Parsed) -> usize {
    let mut sum = 0;
    for range in x.iter().cloned() {
        range.for_each(|id| {
            if !is_valid(id) {
                sum += id;
            }
        })
    }
    sum
}

fn part_2(x: &Parsed) -> usize {
    let mut sum = 0;
    for range in x.iter().cloned() {
        range.for_each(|id| {
            if !is_valid_v2(id) {
                sum += id;
            }
        })
    }
    sum
}

fn is_valid(id: usize) -> bool {
    let digits = digits(id);

    if digits % 2 == 1 {
        return true;
    }

    let divider = 10_usize.pow(digits / 2);

    let top = id / divider;
    let bottom = id % divider;

    top != bottom
}

fn digits(n: usize) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn is_valid_v2(id: usize) -> bool {
    let digits = digits(id);

    !(1..=digits / 2)
        .rev()
        .filter(|x| digits.is_multiple_of(*x))
        .any(|chunk_size| {
            let divider = 10_usize.pow(digits - chunk_size);
            let first_chunk = id / divider;
            let mut rest = id % divider;

            for chunk in 2..digits / chunk_size {
                let divider = 10_usize.pow(digits - chunk_size * chunk);
                let next_chunk = rest / divider;
                if first_chunk != next_chunk {
                    return false;
                }
                rest %= divider;
            }

            rest == first_chunk
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation() {
        assert!(is_valid(123));
        assert!(is_valid(1234));
        assert!(!is_valid(1212));
    }

    #[test]
    fn validation_v2() {
        assert!(is_valid_v2(2121212122));
        assert!(!is_valid_v2(2121212121));
    }
}
