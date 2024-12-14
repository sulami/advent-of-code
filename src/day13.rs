use nom::{
    bytes::complete::tag, character::complete::i64 as parse_u32, multi::separated_list0, IResult,
};

super::solve!("13");

fn parse(input: &str) -> Vec<Machine> {
    let parse_machine = |s| -> IResult<&str, Machine> {
        let (s, _) = tag("Button A: X+")(s)?;
        let (s, a_x) = parse_u32(s)?;
        let (s, _) = tag(", Y+")(s)?;
        let (s, a_y) = parse_u32(s)?;
        let (s, _) = tag("\nButton B: X+")(s)?;
        let (s, b_x) = parse_u32(s)?;
        let (s, _) = tag(", Y+")(s)?;
        let (s, b_y) = parse_u32(s)?;
        let (s, _) = tag("\nPrize: X=")(s)?;
        let (s, prize_x) = parse_u32(s)?;
        let (s, _) = tag(", Y=")(s)?;
        let (s, prize_y) = parse_u32(s)?;
        Ok((
            s,
            Machine {
                a: Coord { x: a_x, y: a_y },
                b: Coord { x: b_x, y: b_y },
                prize: Coord {
                    x: prize_x,
                    y: prize_y,
                },
            },
        ))
    };
    separated_list0(tag("\n\n"), parse_machine)(input)
        .expect("parse failed")
        .1
}

#[derive(Debug, Clone)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Machine {
    a: Coord,
    b: Coord,
    prize: Coord,
}

impl Machine {
    fn tokens_to_win(&self) -> Option<i64> {
        let Self { a, b, prize } = self;

        if 0 == (prize.x * a.y - prize.y * a.x) % (a.y * b.x - a.x * b.y) {
            let b_pushes = (prize.x * a.y - prize.y * a.x) / (a.y * b.x - a.x * b.y);
            if 0 == (prize.x - b_pushes * b.x) % a.x {
                let a_pushes = (prize.x - b_pushes * b.x) / a.x;
                return Some(3 * a_pushes + b_pushes);
            }
        }

        None
    }
}

fn part_1(machines: &[Machine]) -> i64 {
    machines.iter().filter_map(|m| m.tokens_to_win()).sum()
}

fn part_2(machines: &[Machine]) -> i64 {
    machines
        .iter()
        .cloned()
        .filter_map(|mut m| {
            m.prize.x += 10_000_000_000_000;
            m.prize.y += 10_000_000_000_000;
            m.tokens_to_win()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(INPUT)), 480);
    }
}
