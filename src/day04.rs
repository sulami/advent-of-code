use itertools::Itertools;

super::solve!("04");

fn part_1(input: &str) -> usize {
    Search::new(input).count_xmas()
}

fn part_2(input: &str) -> usize {
    Search::new(input).count_x_mas()
}

struct Search {
    width: isize,
    data: Vec<char>,
}

impl Search {
    fn new(s: &str) -> Self {
        let data: Vec<_> = s.chars().filter(|c| "XMAS".contains(*c)).collect();
        let width = s.find('\n').expect("no newline found") as isize;
        Self { width, data }
    }

    fn count_xmas(&self) -> usize {
        let offsets = [
            -self.width - 1,
            -self.width,
            -self.width + 1,
            -1,
            1,
            self.width - 1,
            self.width,
            self.width + 1,
        ];
        let column = |idx| idx % self.width;
        let is_xmas = |start: isize, offset: isize| {
            if ![0, 3].contains(&(column(start)).abs_diff(column(start + 3 * offset))) {
                // Avoid wrapping over the side edges.
                return false;
            };
            (1..=3)
                .filter_map(|step| self.data.get((start + step * offset) as usize))
                .collect::<String>()
                == "MAS"
        };
        self.data
            .iter()
            .positions(|c| *c == 'X')
            .map(|i| {
                offsets
                    .iter()
                    .filter(move |o| is_xmas(i as isize, **o))
                    .count()
            })
            .sum()
    }

    fn count_x_mas(&self) -> usize {
        let offsets = [
            (-self.width - 1, self.width + 1),
            (-self.width + 1, self.width - 1),
        ];
        let is_x_mas = |a: isize| {
            if [0, self.width - 1].contains(&(a % self.width)) {
                return false;
            }
            offsets.iter().all(|(x, y)| {
                [['M', 'S'], ['S', 'M']].contains(&[
                    *self.data.get((a + x) as usize).unwrap_or(&' '),
                    *self.data.get((a + y) as usize).unwrap_or(&' '),
                ])
            })
        };
        self.data
            .iter()
            .positions(|c| *c == 'A')
            .filter(|i| is_x_mas(*i as isize))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 18);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 9);
    }
}
