crate::solve!("01");

type Parsed = Vec<i32>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| l[1..].parse::<i32>().unwrap() * if l.starts_with("L") { -1 } else { 1 })
        .collect()
}

fn part_1(rotations: &Parsed) -> usize {
    rotations
        .iter()
        .fold((50, 0), |(current, zeroes), &r| {
            let new = (1_000 + (current + r)) % 100;
            (new, if current == 0 { zeroes + 1 } else { zeroes })
        })
        .1
}

fn part_2(rotations: &Parsed) -> u32 {
    rotations
        .iter()
        .fold((50, 0), |(mut current, mut zeroes), &r| {
            (0..r.abs()).for_each(|_| {
                current += r.signum();
                match current {
                    0 => zeroes += 1,
                    -1 => current = 99,
                    100 => {
                        current = 0;
                        zeroes += 1;
                    }
                    _ => (),
                }
            });
            (current, zeroes)
        })
        .1
}
