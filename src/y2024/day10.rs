use itertools::Itertools;

crate::solve!("10");

fn parse(input: &str) -> Map {
    let width = input.chars().take_while(char::is_ascii_digit).count();
    let inner = input
        .chars()
        .filter(char::is_ascii_digit)
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
    Map { inner, width }
}

struct Map {
    width: usize,
    inner: Vec<u8>,
}

impl Map {
    fn neighbours(&self, position: usize) -> Vec<usize> {
        let mut rv = vec![];
        if position >= self.width {
            rv.push(position - self.width);
        }
        if position % self.width != 0 {
            rv.push(position - 1);
        }
        if position % self.width != self.width - 1 {
            rv.push(position + 1);
        }
        if position < self.inner.len() - self.width {
            rv.push(position + self.width)
        }
        rv
    }

    fn walk_uphill(&self, position: usize) -> Vec<usize> {
        let height = self.inner[position];
        if height == 9 {
            return vec![position];
        }
        self.neighbours(position)
            .iter()
            .filter(|&p| self.inner[*p] == height + 1)
            .flat_map(|p| self.walk_uphill(*p))
            .collect()
    }
}

fn part_1(map: &Map) -> usize {
    map.inner
        .iter()
        .positions(|&h| h == 0)
        .map(|p| map.walk_uphill(p).iter().unique().count())
        .sum()
}

fn part_2(map: &Map) -> usize {
    map.inner
        .iter()
        .positions(|&h| h == 0)
        .map(|p| map.walk_uphill(p).len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(INPUT)), 36);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&parse(INPUT)), 81);
    }
}
