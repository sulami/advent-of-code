use crate::print_results;
use ahash::HashMap;
use itertools::Itertools;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/24");
    let (initial_values, mappings) = parse_input(input);
    let pt1 = part_1(&initial_values, &mappings);
    print_results(2024, 24, pt1, 0, Some(start))
}

fn part_1(
    initial_values: &HashMap<&str, bool>,
    mappings: &HashMap<&str, (&str, &str, Gate)>,
) -> usize {
    mappings
        .keys()
        .filter(|k| k.starts_with('z'))
        .sorted_unstable()
        .rev()
        .map(|k| wire_value(initial_values, mappings, k.to_string()))
        .inspect(|v| println!("{:?}", v))
        .fold(0, |acc, bit| (acc << 1) | (if bit { 1 } else { 0 }))
}

#[memoize::memoize(CustomHasher: ahash::AHashMap, Ignore: initial_values, Ignore: mappings)]
fn wire_value(
    initial_values: &HashMap<&str, bool>,
    mappings: &HashMap<&str, (&str, &str, Gate)>,
    key: String,
) -> bool {
    if let Some(val) = initial_values.get(key.as_str()) {
        return *val;
    }

    let Some((a, b, gate)) = mappings.get(key.as_str()) else {
        panic!("wire not found")
    };

    let a_val = wire_value(initial_values, mappings, a.to_string());
    let b_val = wire_value(initial_values, mappings, b.to_string());
    match gate {
        Gate::And => a_val && b_val,
        Gate::Or => a_val || b_val,
        Gate::Xor => a_val ^ b_val,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Gate {
    And,
    Or,
    Xor,
}

fn parse_input(s: &str) -> (HashMap<&str, bool>, HashMap<&str, (&str, &str, Gate)>) {
    let initial_values: HashMap<_, _> = s
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (wire, val) = l.split_once(": ").expect("invalid input");
            (wire, val == "1")
        })
        .collect();
    let mappings = s
        .lines()
        .skip(initial_values.len() + 1)
        .map(|l| {
            let (a, gate, b, _, c) = l.split_whitespace().collect_tuple().expect("invalid input");
            let gate = match gate {
                "AND" => Gate::And,
                "OR" => Gate::Or,
                "XOR" => Gate::Xor,
                _ => panic!("invalid gate"),
            };
            (c, (a, b, gate))
        })
        .collect();
    (initial_values, mappings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ahash::HashMapExt;

    #[test]
    fn test_parser() {
        let input = "\
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        // assert_eq!(parse_input(input), (HashMap::new(), HashMap::new()));
    }
}
