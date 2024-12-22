use crate::grid::Coordinate;
use crate::print_results;
use itertools::Itertools;
use std::time::Instant;

pub fn solve() {
    let t = Instant::now();
    let codes = include_str!("inputs/21").lines().collect_vec();
    let pt1 = codes.iter().map(|code| complexity(code)).sum::<usize>();
    // 217676 is too high
    // 212954 is too high, 2 moves off ideal for example 4, but good for the rest
    // 209880 is going to be too low, too low on two examples. That's what get for testing valid upstreams.
    // not 200400
    print_results(2024, 20, pt1, 0, Some(t));
}

fn complexity(code: &str) -> usize {
    let me = Robot::new(RobotKind::ArrowKeys, None); // Me
    let third_robot = Robot::new(RobotKind::ArrowKeys, Some(me)); // -40°
    let second_robot = Robot::new(RobotKind::ArrowKeys, Some(third_robot)); // Radiation
    let mut first_robot = Robot::new(RobotKind::Numpad, Some(second_robot)); // Space
    for c in code.chars() {
        first_robot.press_button(Button::Numpad(c.to_digit(10).unwrap_or(10) as usize));
    }
    first_robot
        .upstream
        .unwrap()
        .upstream
        .unwrap()
        .upstream
        .unwrap()
        .buttons_pressed
        .len()
        * code[..3].parse::<usize>().unwrap()
}

const UP: Coordinate = Coordinate { x: 0, y: -1 };
const DOWN: Coordinate = Coordinate { x: 0, y: 1 };
const LEFT: Coordinate = Coordinate { x: -1, y: 0 };
const RIGHT: Coordinate = Coordinate { x: 1, y: 0 };

#[derive(Debug, Clone, Copy)]
enum RobotKind {
    Numpad,
    ArrowKeys,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Button {
    /// A key on the door numpad. 10 is A.
    Numpad(usize),
    ArrowKey(Coordinate),
    Push,
}

#[derive(Debug, Clone)]
struct Robot {
    kind: RobotKind,
    position: Coordinate,
    buttons_pressed: Vec<Button>,
    upstream: Option<Box<Robot>>,
}

impl Robot {
    fn new(kind: RobotKind, upstream: Option<Robot>) -> Self {
        Self {
            kind,
            upstream: upstream.map(Box::new),
            buttons_pressed: vec![],
            position: match kind {
                RobotKind::Numpad => (2, 3).into(),
                RobotKind::ArrowKeys => (2, 0).into(),
            },
        }
    }

    fn gap(&self) -> Coordinate {
        match self.kind {
            RobotKind::Numpad => (0, 3).into(),
            RobotKind::ArrowKeys => (0, 0).into(),
        }
    }

    /// Returns the button presses needed on the next robot up to reach a button on this one, in
    /// arbitrary order.
    fn upstream_buttons_needed(&self, button: Button) -> Vec<Button> {
        let target = self.button_coordinates(button);
        let mut rv = vec![Button::Push];
        for _ in 0..self.position.x.abs_diff(target.x) {
            rv.push(Button::ArrowKey(
                ((target.x - self.position.x).signum(), 0).into(),
            ))
        }
        // Vertical moves.
        for _ in 0..self.position.y.abs_diff(target.y) {
            rv.push(Button::ArrowKey(
                (0, (target.y - self.position.y).signum()).into(),
            ))
        }
        rv
    }

    /// Returns the buttons required to push be pushed on the next robot up to press a button on
    /// this robot, ordered to minimize the number of button presses upstream. Ends up pressing the
    /// button on this robot and upstream ones.
    fn press_button(&mut self, button: Button) -> Vec<Button> {
        match self.kind {
            RobotKind::Numpad => debug_assert!(matches!(button, Button::Numpad(_))),
            RobotKind::ArrowKeys => {
                debug_assert!(matches!(button, Button::Push | Button::ArrowKey(_)))
            }
        }
        self.buttons_pressed.push(button);
        let target = self.button_coordinates(button);
        let mut upstream_buttons = self.upstream_buttons_needed(button);
        let num_buttons = upstream_buttons.len();
        upstream_buttons = upstream_buttons
            .into_iter()
            // Get all possible orders of buttons.
            .permutations(num_buttons)
            // Ensure we end with the push op.
            .filter(|order| order.last() == Some(&Button::Push))
            // Avoid checking identical sequences several times.
            .unique()
            .filter_map(|sequence| {
                // Calculate the ultimate cost of my pushing buttons for each sequence, filtering
                // out ones that pass through invalid states.
                if let Some(upstream) = &self.upstream {
                    let mut simulated_upstream = upstream.clone();
                    let mut simulated_position = self.position;
                    for button in &sequence {
                        simulated_upstream.press_button(*button);
                        simulated_position += self.button_directions(*button);
                        if simulated_position == self.gap() {
                            return None;
                        }
                    }
                    Some((
                        sequence,
                        simulated_upstream.operator().buttons_pressed.len(),
                    ))
                } else {
                    Some((sequence, 1))
                }
            })
            .sorted_by_key(|(_, cost)| *cost)
            .next()
            .expect("no upstream button sequence found")
            .0;
        self.position = target;
        if let Some(upstream) = self.upstream.as_mut() {
            for b in &upstream_buttons {
                upstream.press_button(*b);
            }
        }
        upstream_buttons
    }

    fn button_coordinates(&self, button: Button) -> Coordinate {
        match button {
            Button::ArrowKey(RIGHT) => (2, 1).into(),
            Button::ArrowKey(LEFT) => (0, 1).into(),
            Button::ArrowKey(DOWN) => (1, 1).into(),
            Button::ArrowKey(UP) => (1, 0).into(),
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

    fn operator(&self) -> Self {
        let mut head = self;
        while let Some(next) = &head.upstream {
            head = &**next;
        }
        head.clone()
    }

    fn button_directions(&self, button: Button) -> Coordinate {
        match button {
            Button::Numpad(_) => unimplemented!("numpad buttons don't have directions"),
            Button::ArrowKey(dir) => dir,
            Button::Push => Coordinate::default(),
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

    /// To match up against the notation used in the examples.
    fn ops_from_str(s: &str) -> Vec<Button> {
        s.chars()
            .map(|c| match c {
                '^' => Button::ArrowKey(UP),
                'v' => Button::ArrowKey(DOWN),
                '>' => Button::ArrowKey(RIGHT),
                '<' => Button::ArrowKey(LEFT),
                'A' => Button::Push,
                _ => panic!("invalid opcode"),
            })
            .collect()
    }

    #[test]
    fn numpad_robot() {
        let mut robot = Robot::new(RobotKind::Numpad, None);
        assert_eq!(robot.press_button(Button::Numpad(10)), vec![Button::Push]);
        assert_eq!(
            robot.press_button(Button::Numpad(0)),
            vec![Button::ArrowKey(LEFT), Button::Push]
        );
        assert_eq!(
            robot.press_button(Button::Numpad(3)),
            vec![Button::ArrowKey(RIGHT), Button::ArrowKey(UP), Button::Push]
        );
    }

    #[test]
    fn arrow_key_robot() {
        let mut robot = Robot::new(RobotKind::ArrowKeys, None);
        assert_eq!(robot.press_button(Button::Push), vec![Button::Push]);
        assert_eq!(
            robot.press_button(Button::ArrowKey(DOWN)),
            vec![Button::ArrowKey(LEFT), Button::ArrowKey(DOWN), Button::Push]
        );
    }

    #[test]
    fn first_level() {
        let me = Robot::new(RobotKind::ArrowKeys, None);
        let mut robot = Robot::new(RobotKind::Numpad, Some(me));
        for c in "029A".chars() {
            robot.press_button(Button::Numpad(c.to_digit(10).unwrap_or(10) as usize));
        }
        assert_eq!(
            robot.operator().buttons_pressed,
            ops_from_str("<A^A>^^AvvvA")
        );
    }

    #[test]
    fn second_level() {
        let me = Robot::new(RobotKind::ArrowKeys, None);
        let second_robot = Robot::new(RobotKind::ArrowKeys, Some(me)); // Radiation
        let mut first_robot = Robot::new(RobotKind::Numpad, Some(second_robot)); // Space
        for c in "029A".chars() {
            first_robot.press_button(Button::Numpad(c.to_digit(10).unwrap_or(10) as usize));
        }
        assert_eq!(
            first_robot.operator().buttons_pressed.len(),
            ops_from_str("v<<A>>^A<A>AvA<^AA>A<vAAA>^A").len()
        );
    }

    #[test]
    fn third_level() {
        let me = Robot::new(RobotKind::ArrowKeys, None);
        let third_robot = Robot::new(RobotKind::ArrowKeys, Some(me)); // -40°
        let second_robot = Robot::new(RobotKind::ArrowKeys, Some(third_robot)); // Radiation
        let mut first_robot = Robot::new(RobotKind::Numpad, Some(second_robot)); // Space
        for c in "029B".chars() {
            first_robot.press_button(Button::Numpad(c.to_digit(10).unwrap_or(10) as usize));
        }
        assert_eq!(
            first_robot.operator().buttons_pressed.len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn part_1_example() {
        assert_eq!(
            INPUT.lines().map(complexity).collect_vec(),
            vec![1972, 58800, 12172, 29184, 24256]
        );
    }
}
