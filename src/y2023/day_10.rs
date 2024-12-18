pub fn solve() {
    let input = include_str!("inputs/day_10");
    let map = Map {
        s: input.as_bytes(),
        width: input.lines().next().unwrap().len() + 1,
    };

    let mut steps = 0;
    let start = map.start();
    let mut position = start;

    // Slightly fudging it here, normally we'd have to find a valid
    // direction to walk into, but that's just a bunch of ifs.
    let mut direction = Direction::Right;

    loop {
        steps += 1;
        position = walk(position, direction);
        if position == start {
            break;
        }
        direction = turn(direction, map.get_field(position));
    }

    println!("{}\n{}", steps / 2, 0)
}

type Coordinates = (usize, usize);

struct Map<'a> {
    s: &'a [u8],
    width: usize,
}

impl<'a> Map<'a> {
    fn get_field(&self, (x, y): Coordinates) -> Field {
        parse_field(self.s[y * self.width + x])
    }

    fn start(&self) -> Coordinates {
        let i = self.s.iter().position(|b| b == &b'S').unwrap();
        (i % self.width, i / self.width)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Horizontal,
    Vertical,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Ground,
    Start,
}

fn parse_field(c: u8) -> Field {
    match c {
        b'|' => Field::Vertical,
        b'-' => Field::Horizontal,
        b'F' => Field::TopLeft,
        b'7' => Field::TopRight,
        b'L' => Field::BottomLeft,
        b'J' => Field::BottomRight,
        b'.' => Field::Ground,
        b'S' => Field::Start,
        _ => panic!("Invalid character"),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn walk((x, y): Coordinates, direction: Direction) -> Coordinates {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn turn(direction: Direction, space: Field) -> Direction {
    match (direction, space) {
        (Direction::Left, Field::Horizontal) => Direction::Left,
        (Direction::Left, Field::TopLeft) => Direction::Down,
        (Direction::Left, Field::BottomLeft) => Direction::Up,
        (Direction::Right, Field::Horizontal) => Direction::Right,
        (Direction::Right, Field::TopRight) => Direction::Down,
        (Direction::Right, Field::BottomRight) => Direction::Up,
        (Direction::Up, Field::Vertical) => Direction::Up,
        (Direction::Up, Field::TopLeft) => Direction::Right,
        (Direction::Up, Field::TopRight) => Direction::Left,
        (Direction::Down, Field::Vertical) => Direction::Down,
        (Direction::Down, Field::BottomLeft) => Direction::Right,
        (Direction::Down, Field::BottomRight) => Direction::Left,
        _ => panic!("Invalid direction"),
    }
}
