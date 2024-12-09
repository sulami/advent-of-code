use itertools::Itertools;

super::solve!("08");

fn parse(input: &str) -> &str {
    input
}

fn part_1(input: &str) -> usize {
    find_antinodes(input, false)
}

fn part_2(input: &str) -> usize {
    find_antinodes(input, true)
}

type Coords = (isize, isize);

fn find_antinodes(input: &str, with_resonance: bool) -> usize {
    let width = input.lines().next().expect("no lines").len();
    let height = input.lines().count();
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices()
                .filter(|(_, name)| *name != '.')
                .map(move |(x, name)| (name, (x as isize, y as isize)))
        })
        .into_group_map()
        .values()
        .flat_map(|antennas| {
            antennas
                .iter()
                .tuple_combinations()
                .flat_map(|(&a, &b)| antinodes_for_pair(width, height, with_resonance, a, b))
        })
        .unique()
        .count()
}

fn antinodes_for_pair(
    width: usize,
    height: usize,
    with_resonance: bool,
    a @ (ax, ay): Coords,
    b @ (bx, by): Coords,
) -> Vec<Coords> {
    let mut nodes = Vec::new();
    let in_bounds = |(x, y)| (0..width as isize).contains(&x) && (0..height as isize).contains(&y);
    let diff_x = ax - bx;
    let diff_y = ay - by;

    if with_resonance {
        let mut candidate = a;
        while in_bounds(candidate) {
            nodes.push(candidate);
            candidate = (candidate.0 + diff_x, candidate.1 + diff_y);
        }
        candidate = b;
        while in_bounds(candidate) {
            nodes.push(candidate);
            candidate = (candidate.0 - diff_x, candidate.1 - diff_y);
        }
    } else {
        let first = (ax + diff_x, ay + diff_y);
        if in_bounds(first) {
            nodes.push(first);
        }
        let second = (bx - diff_x, by - diff_y);
        if in_bounds(second) {
            nodes.push(second);
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(INPUT), 14);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(INPUT), 34);
    }
}
