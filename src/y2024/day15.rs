use rustc_hash::FxHashSet;

crate::solve!("15");

fn parse(input: &str) -> (Map, Coord, Vec<Direction>) {
    let parse_instruction = |c| match c {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => unreachable!(),
    };

    let (map, instructions) = input.split_once("\n\n").expect("no divider");

    let mut walls = FxHashSet::default();
    let mut boxes = FxHashSet::default();
    let mut robot = (0, 0);
    for (y, line) in map.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    walls.insert((x as isize, y as isize));
                }
                'O' => {
                    boxes.insert((x as isize, y as isize));
                }
                '@' => robot = (x as isize, y as isize),
                _ => {}
            }
        }
    }

    let instructions = instructions
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(parse_instruction)
        .collect();

    (Map { walls, boxes }, robot, instructions)
}

fn part_1((map, robot, instructions): &(Map, Coord, Vec<Direction>)) -> isize {
    let mut map = map.to_owned();
    let mut robot = robot.to_owned();

    'ins: for instruction in instructions {
        let diff = instruction.diff();
        let mut boxes_to_move = vec![];
        let mut current = robot;
        'bxs: loop {
            let next = add_coords(current, diff);
            if map.walls.contains(&next) {
                // Hit a wall, abort this instruction.
                continue 'ins;
            }
            if map.boxes.contains(&next) {
                // Need to move more boxes.
                boxes_to_move.push(next);
                current = add_coords(current, diff);
            } else {
                // Found empty space.
                break 'bxs;
            }
        }
        // Two-pass to avoid removing boxes we just added.
        for b in boxes_to_move.iter() {
            map.boxes.remove(b);
        }
        for b in boxes_to_move {
            map.boxes.insert(add_coords(b, diff));
        }
        robot = add_coords(robot, diff);
    }

    map.boxes.iter().map(|b| b.0 + b.1 * 100).sum()
}

fn part_2((map, robot, instructions): &(Map, Coord, Vec<Direction>)) -> isize {
    let mut map = map.to_owned();
    let mut robot = robot.to_owned();

    // Shift everything over.
    robot.0 *= 2;
    // Boxes still identified by their left coordinate.
    map.boxes = map.boxes.iter().map(|(x, y)| (x * 2, *y)).collect();
    // Walls are just duplicated for simplicity.
    map.walls = map
        .walls
        .iter()
        .flat_map(|(x, y)| [(x * 2, *y), (x * 2 + 1, *y)])
        .collect();

    // Same as part 1, but with extra box shifting.
    'ins: for instruction in instructions {
        let diff = instruction.diff();
        let mut boxes_to_move = FxHashSet::default();
        let mut pushed_on = FxHashSet::from_iter([add_coords(robot, diff)]);
        'bxs: loop {
            if pushed_on.is_empty() {
                // Everything is free, we can push this way.
                break 'bxs;
            }
            // Are we pushing onto a wall anywhere?
            if !pushed_on.is_disjoint(&map.walls) {
                // Hit a wall, abort this instruction.
                continue 'ins;
            }
            let mut new_pushed_on = FxHashSet::default();
            for &centre in pushed_on.iter() {
                let left_of_centre = add_coords(centre, (-1, 0));
                let right_of_centre = add_coords(centre, (1, 0));

                // Do we hit a box, and if so where?
                // Depends on the direction we push on as well.
                match instruction {
                    Direction::Up | Direction::Down => {
                        if map.boxes.contains(&centre) {
                            // Hit the left side of the box.
                            boxes_to_move.insert(centre);
                            new_pushed_on.insert(add_coords(centre, diff));
                            new_pushed_on.insert(add_coords(right_of_centre, diff));
                        } else if map.boxes.contains(&left_of_centre) {
                            // Hit the right side of the box.
                            boxes_to_move.insert(left_of_centre);
                            new_pushed_on.insert(add_coords(left_of_centre, diff));
                            new_pushed_on.insert(add_coords(centre, diff));
                        };
                    }
                    Direction::Right => {
                        if map.boxes.contains(&centre) {
                            boxes_to_move.insert(centre);
                            new_pushed_on.insert(add_coords(right_of_centre, diff));
                        };
                    }
                    Direction::Left => {
                        if map.boxes.contains(&left_of_centre) {
                            boxes_to_move.insert(left_of_centre);
                            new_pushed_on.insert(add_coords(left_of_centre, diff));
                        }
                    }
                };
            }
            pushed_on = new_pushed_on;
        }
        // Two-pass to avoid removing boxes we just added.
        for b in boxes_to_move.iter() {
            map.boxes.remove(b);
        }
        for b in boxes_to_move {
            map.boxes.insert(add_coords(b, diff));
        }
        robot = add_coords(robot, diff);
    }

    map.boxes.iter().map(|b| b.0 + b.1 * 100).sum()
}

type Coord = (isize, isize);

fn add_coords(a: Coord, b: Coord) -> Coord {
    (a.0 + b.0, a.1 + b.1)
}

#[derive(Clone)]
struct Map {
    walls: FxHashSet<Coord>,
    boxes: FxHashSet<Coord>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn diff(&self) -> Coord {
        match self {
            Self::Right => (1, 0),
            Self::Left => (-1, 0),
            Self::Up => (0, -1),
            Self::Down => (0, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(INPUT)), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&parse(INPUT)), 9021);
    }
}
