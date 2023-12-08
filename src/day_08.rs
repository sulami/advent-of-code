#![cfg_attr(not(feature = "day-08"), allow(dead_code))]

pub fn solve() -> (u64, u64) {
    let input = include_str!("../inputs/day_08");

    let part_1 = navigate(input, "AAA", |pos| pos == "ZZZ");
    let part_2 = input
        .lines()
        .skip(2)
        .filter(|line| &line[2..3] == "A")
        .map(|line| navigate(input, &line[..3], |pos| &pos[2..] == "Z"))
        .fold(1, lcm);

    (part_1, part_2)
}

fn navigate(input: &str, start: &str, pred: impl Fn(&str) -> bool) -> u64 {
    let mut route = input.lines().next().unwrap().chars().cycle();
    let mut position = start;
    let mut steps = 0;
    while !pred(position) {
        steps += 1;
        match route.next() {
            Some('L') => {
                position = &input
                    .lines()
                    .find(|line| line.starts_with(position))
                    .unwrap()[7..10];
            }
            Some('R') => {
                position = &input
                    .lines()
                    .find(|line| line.starts_with(position))
                    .unwrap()[12..15];
            }
            _ => panic!("invalid directon"),
        }
    }
    steps
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        core::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
