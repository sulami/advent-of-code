use crate::coordinate::Coordinate;
use ahash::HashMap;

crate::solve!("04");

type Parsed = HashMap<Coordinate, ()>;

fn parse(input: &str) -> Parsed {
    Coordinate::parse_grid(input, |c| if c == '@' { Some(()) } else { None })
}

fn part_1(x: &Parsed) -> usize {
    freeable(x).len()
}

fn part_2(x: &Parsed) -> usize {
    let mut x = x.clone();

    let mut count = 0;
    while let freeable = freeable(&x) && !freeable.is_empty() {
        count += freeable.len();
        for c in freeable {
            x.remove(&c);
        }
    }
    count
}

#[inline]
fn freeable(stacks: &HashMap<Coordinate, ()>) -> Vec<Coordinate> {
    stacks
        .keys()
        .filter(|c| {
            c.diagonal_neighbours(..)
                .iter()
                .filter(|c| stacks.contains_key(c))
                .count()
                < 4
        })
        .copied()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse(INPUT)), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(INPUT)), 43);
    }
}
