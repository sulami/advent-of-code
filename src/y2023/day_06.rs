#![cfg_attr(not(feature = "day-06"), allow(dead_code))]

use arrayvec::ArrayString;

pub fn solve() -> (u64, u64) {
    let input = include_str!("../inputs/day_06");

    let times = input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap());
    let distances = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap());

    let races = times.zip(distances);

    let part_1 = races
        .map(|(time, distance)| {
            (0..=time)
                .filter(|push_duration| wins_race(time, *push_duration, distance))
                .count() as u64
        })
        .product();

    let mut buffer_str: ArrayString<64> = ArrayString::new();
    input
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .for_each(|s| buffer_str.push_str(s));
    let race_time: u64 = buffer_str.parse().unwrap();

    buffer_str.clear();
    input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .for_each(|s| buffer_str.push_str(s));
    let race_distance: u64 = buffer_str.parse().unwrap();

    let losing_too_short = (0..=race_time)
        .find(|push_duration| wins_race(race_time, *push_duration, race_distance))
        .unwrap();

    let losing_too_long = (0..=race_time)
        .rev()
        .find(|push_duration| wins_race(race_time, *push_duration, race_distance))
        .unwrap();

    let part_2 = losing_too_long - losing_too_short + 1;
    (part_1, part_2)
}

#[inline]
fn wins_race(race_duration: u64, push_duration: u64, record: u64) -> bool {
    let time_moving = race_duration - push_duration;
    let velocity = push_duration;
    let distance = time_moving * velocity;
    distance >= record
}
