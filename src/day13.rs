use nom::{
    bytes::complete::tag, character::complete::u64 as parse_u32, multi::separated_list0, IResult,
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
    x: u64,
    y: u64,
}

#[derive(Debug, Clone)]
struct Machine {
    a: Coord,
    b: Coord,
    prize: Coord,
}

impl Machine {
    fn tokens_to_win(&self) -> Option<u64> {
        let mut tokens = None;

        // Don't even ask.
        if self.prize.y * self.a.x <= self.prize.x * self.a.y
            && (self.prize.x * self.a.y - self.prize.y * self.a.x)
                % (self.a.y * self.b.x - self.a.x * self.b.y)
                == 0
        {
            let b_pushes = (self.prize.x * self.a.y - self.prize.y * self.a.x)
                / (self.a.y * self.b.x - self.a.x * self.b.y);
            if (self.prize.x - b_pushes * self.b.x) % self.a.x == 0 {
                let a_pushes = (self.prize.x - b_pushes * self.b.x) / self.a.x;
                tokens = Some(3 * a_pushes + b_pushes);
            }
        }

        if self.prize.y * self.b.x <= self.prize.x * self.b.y
            && (self.prize.x * self.b.y - self.prize.y * self.b.x)
                % (self.a.x * self.b.y - self.a.y * self.b.x)
                == 0
        {
            let a_pushes = (self.prize.x * self.b.y - self.prize.y * self.b.x)
                / (self.a.x * self.b.y - self.a.y * self.b.x);
            if (self.prize.x - a_pushes * self.a.x) % self.b.x == 0 {
                let b_pushes = (self.prize.x - a_pushes * self.a.x) / self.b.x;
                tokens = tokens.map_or(Some(3 * a_pushes + b_pushes), |t| {
                    Some(t.min(3 * a_pushes + b_pushes))
                });
            }
        }

        tokens
    }
}

fn part_1(machines: &[Machine]) -> u64 {
    machines.iter().filter_map(|m| m.tokens_to_win()).sum()
}

fn part_2(machines: &[Machine]) -> u64 {
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
