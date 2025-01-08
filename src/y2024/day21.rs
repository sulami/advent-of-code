use crate::coordinate::Coordinate;
use crate::{coordinate, print_results};
use itertools::Itertools;
use std::time::Instant;

pub fn solve() {
    let t = Instant::now();
    let codes = include_str!("inputs/21").lines().collect_vec();
    let pt1 = codes.iter().map(|code| complexity(code, 3)).sum::<usize>();
    let pt2 = codes.iter().map(|code| complexity(code, 26)).sum::<usize>();
    print_results(2024, 20, pt1, pt2, Some(t));
}

fn complexity(code: &str, operators: usize) -> usize {
    let mut cost = 0;
    let mut position = Coordinate::new(2, 3);
    for c in code.chars() {
        let button = Button::Numpad(c.to_digit(10).unwrap_or(10) as usize);
        cost += button_cost(position, (0, 3).into(), button, operators);
        position = button_coordinates(button);
    }
    cost * code[..3].parse::<usize>().unwrap()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Button {
    /// A key on the door numpad. 10 is A.
    Numpad(usize),
    ArrowKey(Coordinate),
    Push,
}

#[memoize::memoize(CustomHasher: ahash::AHashMap)]
fn button_cost(from_position: Coordinate, gap: Coordinate, button: Button, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    let upstream_buttons = upstream_buttons_needed(from_position, button_coordinates(button));
    let k = upstream_buttons.len();
    upstream_buttons
        .into_iter()
        // Get all possible orders of buttons.
        .permutations(k)
        // Ensure we end with the push op.
        .filter(|order| order.last() == Some(&Button::Push))
        // Avoid checking identical sequences several times.
        .unique()
        // Get the upstream cost for each sequence.
        .filter_map(|sequence| {
            let mut this_position = from_position;
            let mut upstream_position = Coordinate::new(2, 0);
            let mut cost = 0;
            for next_button in &sequence {
                this_position += button_directions(*next_button);
                if this_position == gap {
                    // Avoid sequences that would lead to invalid moves on this robot.
                    return None;
                }
                cost += button_cost(
                    upstream_position,
                    Coordinate::default(),
                    *next_button,
                    depth - 1,
                );
                upstream_position = button_coordinates(*next_button);
            }
            Some((sequence, cost))
        })
        .sorted_unstable_by_key(|(_, cost)| *cost)
        .next()
        .expect("no sequence found")
        .1
}

/// Returns the button presses needed on the next robot up to reach a button on this one, in
/// arbitrary order.
fn upstream_buttons_needed(from: Coordinate, to: Coordinate) -> Vec<Button> {
    let mut buttons = vec![Button::Push];
    for _ in 0..from.x.abs_diff(to.x) {
        buttons.push(Button::ArrowKey(((to.x - from.x).signum(), 0).into()))
    }
    for _ in 0..from.y.abs_diff(to.y) {
        buttons.push(Button::ArrowKey((0, (to.y - from.y).signum()).into()))
    }
    buttons
}

/// Returns the direction of a movement on this robot a button pressed on the next robot up will
/// result in.
fn button_directions(button: Button) -> Coordinate {
    match button {
        Button::Numpad(_) => unimplemented!("numpad buttons don't have directions"),
        Button::ArrowKey(dir) => dir,
        Button::Push => Coordinate::default(),
    }
}

/// Returns the coordinates of the button in question.
fn button_coordinates(button: Button) -> Coordinate {
    match button {
        Button::ArrowKey(coordinate::RIGHT) => (2, 1).into(),
        Button::ArrowKey(coordinate::LEFT) => (0, 1).into(),
        Button::ArrowKey(coordinate::DOWN) => (1, 1).into(),
        Button::ArrowKey(coordinate::UP) => (1, 0).into(),
        Button::ArrowKey(_) => panic!("invalid arrow key"),
        Button::Push => (2, 0).into(),
        Button::Numpad(n) => {
            [
                (1, 3), // 0
                (0, 2), // 1
                (1, 2), // 2
                (2, 2), // 3
                (0, 1), // 4
                (1, 1), // 5
                (2, 1), // 6
                (0, 0), // 7
                (1, 0), // 8
                (2, 0), // 9
                (2, 3), // A
            ]
            .map(Coordinate::from)[n]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
029A
980A
179A
456A
379A";

    #[test]
    fn test_simple() {
        assert_eq!(complexity("029A", 1), 12 * 29);
    }

    #[test]
    fn part_1_example() {
        assert_eq!(
            INPUT.lines().map(|code| complexity(code, 3)).collect_vec(),
            vec![1972, 58800, 12172, 29184, 24256]
        );
    }
}
