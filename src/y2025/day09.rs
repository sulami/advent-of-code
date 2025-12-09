use crate::coordinate::Coordinate;
use ahash::{HashMap, HashSet};
use itertools::Itertools;

crate::solve!("09");

type Parsed = Vec<Coordinate>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap()).into()
        })
        .collect()
}

fn part_1(x: &Parsed) -> usize {
    x.iter()
        .tuple_combinations()
        .map(|(a, b)| (1 + a.x.abs_diff(b.x)) * (1 + a.y.abs_diff(b.y)))
        .max()
        .unwrap()
}

fn part_2(x: &Parsed) -> usize {
    // Draw straight lines between red tiles.
    let path = x
        .iter()
        .circular_tuple_windows()
        .flat_map(|(a, b)| {
            if a.x == b.x {
                (a.y.min(b.y) + 1..a.y.max(b.y))
                    .map(|y| Coordinate::new(a.x, y))
                    .collect_vec()
            } else {
                (a.x.min(b.x) + 1..a.x.max(b.x))
                    .map(|x| Coordinate::new(x, a.y))
                    .collect_vec()
            }
        })
        .collect::<HashSet<_>>();

    // Broadly the same as part 1, but exclude any rectangles that intersect with the path.
    x.iter()
        .tuple_combinations()
        // Sort first, much faster.
        .sorted_unstable_by_key(|(a, b)| (1 + a.x.abs_diff(b.x)) * (1 + a.y.abs_diff(b.y)))
        .rev()
        .find(|(a, b)| {
            !path.iter().any(|c| {
                (a.x.min(b.x) + 1..a.x.max(b.x)).contains(&c.x)
                    && (a.y.min(b.y) + 1..a.y.max(b.y)).contains(&c.y)
            })
        })
        .map(|(a, b)| (1 + a.x.abs_diff(b.x)) * (1 + a.y.abs_diff(b.y)))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse(INPUT)), 50);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(INPUT)), 24);
    }
}
