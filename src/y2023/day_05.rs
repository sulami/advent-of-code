#![cfg_attr(not(feature = "day-05"), allow(dead_code))]

//! This is really slow for part 2, but smarter range analysis would
//! be tricky with our memory constraints. It does work though, the
//! solution was computed on real hardware.

use core::ops::Range;

use arrayvec::ArrayVec;

pub fn solve() -> (u64, u64) {
    let input = include_str!("../inputs/day_05");

    let part_1 = input.lines().next().unwrap()[7..]
        .split_whitespace()
        .map(|seed| seed_to_location(input, seed.parse().unwrap()))
        .min()
        .unwrap_or(u64::MAX);

    let mut seed_ranges: ArrayVec<Range<u64>, 10> = ArrayVec::new();
    let mut seed_nums = input.lines().next().unwrap()[7..].split_whitespace();
    while let Some(start) = seed_nums.next() {
        let start: u64 = start.parse().unwrap();
        let range: u64 = seed_nums.next().unwrap().parse().unwrap();
        seed_ranges.push(Range {
            start,
            end: start + range,
        });
    }

    let part_2 = (0..)
        .map(|location| (location, location_to_seed(input, location)))
        .find(|(_, seed)| seed_ranges.iter().any(|r| r.contains(seed)))
        .map(|(location, _)| location)
        .unwrap();

    (part_1, part_2)
}

fn seed_to_location(input: &str, seed: u64) -> u64 {
    let mut current = seed;
    let mut maps = input.lines().skip(3);

    while let Some(line) = maps.next() {
        if line.is_empty() {
            // No matches, current unchanged, move on to next map,
            // skip the header row.
            maps.next();
            continue;
        }

        let mut triplet = line.split_whitespace();

        let destination_start: u64 = triplet.next().unwrap().parse().unwrap();
        let source_start: u64 = triplet.next().unwrap().parse().unwrap();
        let range: u64 = triplet.next().unwrap().parse().unwrap();

        if source_start <= current && current < source_start + range {
            // Match, update current, skip forward to the next map, skip the header row.
            current = destination_start + current - source_start;
            while !maps.next().unwrap_or_default().is_empty() {}
            maps.next();
        }
    }

    current
}

fn location_to_seed(input: &str, location: u64) -> u64 {
    let mut current = location;
    let mut maps = input.lines().rev();

    while let Some(line) = maps.next() {
        if !line.chars().next().unwrap().is_ascii_digit() {
            // Reached header, skip forward.
            maps.next();
            continue;
        }

        let mut triplet = line.split_whitespace();

        let destination_start: u64 = triplet.next().unwrap().parse().unwrap();
        let source_start: u64 = triplet.next().unwrap().parse().unwrap();
        let range: u64 = triplet.next().unwrap().parse().unwrap();

        if destination_start <= current && current < destination_start + range {
            // Match, update current, skip forward to the next map.
            current = source_start + current - destination_start;
            while !maps.next().unwrap_or_default().is_empty() {}
        }
    }

    current
}
