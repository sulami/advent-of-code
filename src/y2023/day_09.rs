pub fn solve() {
    let input = include_str!("inputs/day_09");

    let part_1: i64 = input
        .lines()
        .map(|line| extrapolate(line, Direction::Back))
        .sum();
    let part_2: i64 = input
        .lines()
        .map(|line| extrapolate(line, Direction::Front))
        .sum();

    println!("{}\n{}", part_1 as u64, part_2 as u64)
}

fn extrapolate(line: &str, direction: Direction) -> i64 {
    let mut derivations: Vec<Vec<i64>> = Vec::new();
    derivations.push(Vec::new());

    line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .for_each(|n| derivations[0].push(n));

    let mut this_level = Vec::new();
    while derivations.last().unwrap().iter().any(|n| *n != 0) {
        derivations
            .last()
            .unwrap()
            .iter()
            .copied()
            .derivative()
            .for_each(|n| {
                this_level.push(n);
            });
        derivations.push(this_level.clone());
        this_level.clear();
    }

    let mut diff: i64 = 0;
    for level in derivations.iter_mut().rev().skip(1) {
        match direction {
            Direction::Front => {
                let val = level.first().unwrap() - diff;
                level.insert(0, val);
                diff = val;
            }
            Direction::Back => {
                let val = level.last().unwrap() + diff;
                level.push(val);
                diff = val;
            }
        }
    }

    match direction {
        Direction::Front => *derivations[0].first().unwrap(),
        Direction::Back => *derivations[0].last().unwrap(),
    }
}

enum Direction {
    Front,
    Back,
}

trait Differential {
    fn derivative(self) -> Derivative<Self>
    where
        Self: Iterator + Sized;
}

struct Derivative<I>
where
    I: Iterator,
{
    iter: I,
    last: Option<I::Item>,
}

impl<I> Iterator for Derivative<I>
where
    I: Iterator,
    I::Item: Copy + core::ops::Sub<Output = I::Item>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.last.is_none() {
            match self.iter.next() {
                None => return None,
                Some(item) => {
                    self.last = Some(item);
                }
            }
        }
        match self.iter.next() {
            None => None,
            Some(item) => {
                let dx = item - self.last.unwrap();
                self.last = Some(item);
                Some(dx)
            }
        }
    }
}

impl<I> Differential for I
where
    I: Iterator + Sized,
{
    fn derivative(self) -> Derivative<Self> {
        Derivative {
            iter: self,
            last: None,
        }
    }
}
