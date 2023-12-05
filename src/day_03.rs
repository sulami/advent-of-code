#![cfg_attr(not(feature = "day-03"), allow(dead_code))]

use nom::{
    character::complete::{none_of, u32},
    combinator::consumed,
    multi::many0_count,
};

pub fn solve() -> (u64, u64) {
    let input = include_str!("../inputs/day_03");

    // Part 1
    let mut sum = 0;

    for (y, mut line) in input.lines().enumerate() {
        let mut x = 0;

        loop {
            // Skip past non-numbers, record the x position.
            let (rest, offset) =
                many0_count(none_of::<&str, &str, ()>("0123456789"))(line).unwrap();
            x += offset;

            // Next line if done with this one.
            if rest.is_empty() {
                break;
            }

            // Grab a number, record that x position as well.
            let (rest, (string, number)) = consumed(u32::<&str, ()>)(rest).unwrap();

            // If the number is a part number, add it up.
            if is_part_number(input, x, y, string.len()) {
                sum += number;
            }

            // Continue reading from here.
            x += string.len();
            line = rest;
        }
    }

    // Part 2
    let mut sum2 = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '*' {
                sum2 += gear_ratio(input, x, y);
            }
        }
    }

    (sum as u64, sum2 as u64)
}

fn is_part_number(input: &str, x: usize, y: usize, width: usize) -> bool {
    // Bounding box x coordinate.
    let start = x.saturating_sub(1);
    // Bounding box width.
    let box_width = if x == 0 { width + 1 } else { width + 2 };

    // Previous line, if any.
    if y > 0 {
        let line = input.lines().nth(y - 1).unwrap();
        if line.chars().skip(start).take(box_width).any(|c| c != '.') {
            return true;
        }
    }

    // Next line, if any.
    if y + 1 < input.lines().count() {
        let line = input.lines().nth(y + 1).unwrap();
        if line.chars().skip(start).take(box_width).any(|c| c != '.') {
            return true;
        }
    }

    // Same line.
    let line = input.lines().nth(y).unwrap();
    // Left side, if any.
    if x > 0 && line.chars().nth(start).unwrap() != '.' {
        return true;
    }
    // Right side, if any.
    if x + width + 1 < line.len() && line.chars().nth(x + width).unwrap() != '.' {
        return true;
    }

    false
}

fn gear_ratio(input: &str, x: usize, y: usize) -> u32 {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    // There are eight surrounding fields (or less). We have eight
    // bits in a u8. We can use a bitmap to see where we have numbers
    // surrounding this gear.
    let mut neighbours = 0b_0000_0000_u8;

    // Top row.
    if x > 0 && y > 0 && is_number(input, x - 1, y - 1) {
        neighbours |= 0b_1000_0000;
    }
    if y > 0 && is_number(input, x, y - 1) {
        neighbours |= 0b_0100_0000;
    }
    if x + 1 < width && y > 0 && is_number(input, x + 1, y - 1) {
        neighbours |= 0b_0010_0000;
    }

    // Either side.
    if x > 0 && is_number(input, x - 1, y) {
        neighbours |= 0b_0001_0000;
    }
    if x + 1 < width && is_number(input, x + 1, y) {
        neighbours |= 0b_0000_1000;
    }

    // Bottom row.
    if x > 0 && y + 1 < height && is_number(input, x - 1, y + 1) {
        neighbours |= 0b_0000_0100;
    }
    if y + 1 < height && is_number(input, x, y + 1) {
        neighbours |= 0b_0000_0010;
    }
    if x + 1 < width && y + 1 < height && is_number(input, x + 1, y + 1) {
        neighbours |= 0b_0000_0001;
    }

    // We can now count how many surrounding numbers we actually have.
    let mut numbers = 0;

    // Top row.
    if neighbours & 0b_1000_0000 != 0
        && neighbours & 0b_0010_0000 != 0
        && neighbours & 0b_0100_0000 == 0
    {
        // Two on top, ono either side.
        numbers += 2;
    } else if neighbours & 0b_1110_0000 != 0 {
        // One on top, somewhere.
        numbers += 1;
    }

    // Left side.
    if neighbours & 0b_0001_0000 != 0 {
        numbers += 1;
    }

    // Right side.
    if neighbours & 0b_0000_1000 != 0 {
        numbers += 1;
    }

    // Bottom row.
    if neighbours & 0b_0000_0100 != 0
        && neighbours & 0b_0000_0001 != 0
        && neighbours & 0b_0000_0010 == 0
    {
        // Two on bottom, ono either side.
        numbers += 2;
    } else if neighbours & 0b_0000_0111 != 0 {
        // One on bottom, somewhere.
        numbers += 1;
    }

    if numbers != 2 {
        // Not exactly two neighbouring numbers, not a gear.
        return 0;
    }

    let mut ratio = 1;

    // Now to parsing those numbers.

    // Top row.
    if neighbours & 0b_1000_0000 != 0
        && neighbours & 0b_0010_0000 != 0
        && neighbours & 0b_0100_0000 == 0
    {
        // If we have the split case on top, scan left for the left
        // number. The right number is easy, we know where it starts.
        let line = input.lines().nth(y - 1).unwrap();

        // Left
        let mut start = x - 1;
        while start > 0 && line.chars().nth(start - 1).unwrap().is_ascii_digit() {
            start -= 1;
        }
        ratio *= u32::<&str, ()>(&line[start..]).unwrap().1;

        // Right
        ratio *= u32::<&str, ()>(&line[(x + 1)..]).unwrap().1;
    } else if neighbours & 0b_1000_0000 != 0 {
        // Otherwise if we have a number that covers the left side,
        // scan left for the start.
        let line = input.lines().nth(y - 1).unwrap();

        // Left
        let mut start = x - 1;
        while start > 0 && line.chars().nth(start - 1).unwrap().is_ascii_digit() {
            start -= 1;
        }
        ratio *= u32::<&str, ()>(&line[start..]).unwrap().1;
    } else if neighbours & 0b_0110_0000 != 0 {
        // Otherwise if we have a number that starts either in the
        // centre or on the right, we can start in the centre,
        // potentially discarding the first character.
        let line = input.lines().nth(y - 1).unwrap();

        if line.chars().nth(x).unwrap().is_ascii_digit() {
            // Start in the centre.
            ratio *= u32::<&str, ()>(&line[x..]).unwrap().1;
        } else {
            // Start on the right.
            ratio *= u32::<&str, ()>(&line[(x + 1)..]).unwrap().1;
        }
    }

    // Left side we have to scan backwards to the start of the number.
    if neighbours & 0b_0001_0000 != 0 {
        let line = input.lines().nth(y).unwrap();
        let mut start = x - 1;
        while start > 0 && line.chars().nth(start - 1).unwrap().is_ascii_digit() {
            start -= 1;
        }
        ratio *= u32::<&str, ()>(&line[start..]).unwrap().1;
    }

    // Right side is easy, we know where it starts.
    if neighbours & 0b_0000_1000 != 0 {
        let line = input.lines().nth(y).unwrap();
        ratio *= u32::<&str, ()>(&line[(x + 1)..]).unwrap().1;
    }

    // Bottom row.
    if neighbours & 0b_0000_0100 != 0
        && neighbours & 0b_0000_0001 != 0
        && neighbours & 0b_0000_0010 == 0
    {
        // If we have the split case on the bottom, scan left for the
        // left number. The right number is easy, we know where it
        // starts.
        let line = input.lines().nth(y + 1).unwrap();

        // Left
        let mut start = x - 1;
        while start > 0 && line.chars().nth(start - 1).unwrap().is_ascii_digit() {
            start -= 1;
        }
        ratio *= u32::<&str, ()>(&line[start..]).unwrap().1;

        // Right
        ratio *= u32::<&str, ()>(&line[(x + 1)..]).unwrap().1;
    } else if neighbours & 0b_0000_0100 != 0 {
        // Otherwise if we have a number that covers the left side,
        // scan left for the start.
        let line = input.lines().nth(y + 1).unwrap();

        // Left
        let mut start = x - 1;
        while start > 0 && line.chars().nth(start - 1).unwrap().is_ascii_digit() {
            start -= 1;
        }
        ratio *= u32::<&str, ()>(&line[start..]).unwrap().1;
    } else if neighbours & 0b_0000_0011 != 0 {
        // Otherwise if we have a number that starts either in the
        // centre or on the right, we can start in the centre,
        // potentially discarding the first character.
        let line = input.lines().nth(y + 1).unwrap();

        if line.chars().nth(x).unwrap().is_ascii_digit() {
            // Start in the centre.
            ratio *= u32::<&str, ()>(&line[x..]).unwrap().1;
        } else {
            // Start on the right.
            ratio *= u32::<&str, ()>(&line[(x + 1)..]).unwrap().1;
        }
    }

    ratio
}

fn is_number(input: &str, x: usize, y: usize) -> bool {
    input
        .lines()
        .nth(y)
        .unwrap()
        .chars()
        .nth(x)
        .unwrap()
        .is_ascii_digit()
}
