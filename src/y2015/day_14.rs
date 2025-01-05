use crate::print_results;
use itertools::Itertools;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/14");
    let reindeer = parse(input);
    let pt1 = reindeer
        .iter()
        .map(|r| r.position_after(2503))
        .max()
        .unwrap();
    let pt2 = (1..=2503)
        .flat_map(|t| reindeer.iter().max_set_by_key(|r| r.position_after(t)))
        .counts_by(|r| r.name)
        .into_values()
        .max()
        .unwrap();
    print_results(2015, 14, pt1, pt2, Some(start));
}

struct Reindeer<'a> {
    name: &'a str,
    speed: u32,
    endurance: u32,
    rest: u32,
}

impl Reindeer<'_> {
    fn position_after(&self, time: u32) -> u32 {
        (time / (self.endurance + self.rest)) * self.speed * self.endurance
            + (time % (self.endurance + self.rest)).min(self.endurance) * self.speed
    }
}

fn parse(s: &str) -> Vec<Reindeer> {
    s.lines()
        .map(|l| {
            let words = l.split_whitespace().collect_vec();
            Reindeer {
                name: words[0],
                speed: words[3].parse().expect("invalid speed"),
                endurance: words[6].parse().expect("invalid endurance"),
                rest: words[13].parse().expect("invalid rest"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reindeer_position() {
        let comet = Reindeer {
            name: "comet",
            speed: 14,
            endurance: 10,
            rest: 127,
        };
        assert_eq!(comet.position_after(1), 14);
        assert_eq!(comet.position_after(10), 140);
        assert_eq!(comet.position_after(11), 140);
        assert_eq!(comet.position_after(138), 154);
    }
}
