use nom::{
    character::complete::{space1, u64 as parse_u64},
    multi::separated_list1,
};

crate::solve!("11");

fn parse(input: &str) -> Vec<u64> {
    separated_list1(space1::<&str, ()>, parse_u64)(input)
        .expect("invalid stones")
        .1
}

fn part_1(stones: &[u64]) -> usize {
    stones.iter().map(|&s| stone_size((s, 25))).sum()
}

fn part_2(stones: &[u64]) -> usize {
    stones.iter().map(|&s| stone_size((s, 75))).sum()
}

#[memoize::memoize(CustomHasher: rustc_hash::FxHashMap, HasherInit: rustc_hash::FxHashMap::default())]
fn stone_size(key: (u64, usize)) -> usize {
    let (stone, iterations) = key;
    if iterations == 0 {
        return 1;
    }

    if stone == 0 {
        return stone_size((1, iterations - 1));
    }

    let digits = stone.checked_ilog10().unwrap_or(0) + 1;
    if digits % 2 == 0 {
        let a = stone_size((stone / 10_u64.pow(digits / 2), iterations - 1));
        let b = stone_size((stone % 10_u64.pow(digits / 2), iterations - 1));
        return a + b;
    }

    stone_size((stone * 2024, iterations - 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "125 17";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(INPUT)), 55312);
    }
}
