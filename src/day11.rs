use nom::{
    character::complete::{space1, u64 as parse_u64},
    multi::separated_list1,
};
use rustc_hash::FxHashMap;

super::solve!("11");

fn parse(input: &str) -> Vec<u64> {
    separated_list1(space1::<&str, ()>, parse_u64)(input)
        .expect("invalid stones")
        .1
}

fn part_1(stones: &[u64]) -> usize {
    let mut cache = FxHashMap::default();
    stones
        .iter()
        .map(|&s| stone_size((s, 25), &mut cache))
        .sum()
}

fn part_2(stones: &[u64]) -> usize {
    let mut cache = FxHashMap::default();
    stones
        .iter()
        .map(|&s| stone_size((s, 75), &mut cache))
        .sum()
}

fn stone_size(
    key @ (stone, iterations): (u64, usize),
    cache: &mut FxHashMap<(u64, usize), usize>,
) -> usize {
    if iterations == 0 {
        return 1;
    }
    cache.get(&key).map(ToOwned::to_owned).unwrap_or_else(|| {
        if stone == 0 {
            let rv = stone_size((1, iterations - 1), cache);
            cache.insert(key, rv);
            return rv;
        }

        let digits = stone.checked_ilog10().unwrap_or(0) + 1;
        if digits % 2 == 0 {
            let a = stone_size((stone / 10_u64.pow(digits / 2), iterations - 1), cache);
            let b = stone_size((stone % 10_u64.pow(digits / 2), iterations - 1), cache);
            let rv = a + b;
            cache.insert(key, rv);
            return rv;
        }

        let rv = stone_size((stone * 2024, iterations - 1), cache);
        cache.insert(key, rv);
        rv
    })
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
