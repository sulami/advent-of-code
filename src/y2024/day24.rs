use crate::print_results;
use ahash::{HashMap, HashMapExt};
use itertools::Itertools;
use rayon::prelude::*;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/24");
    let (initial_values, mappings) = parse_input(input);
    let pt1 = part_1(&initial_values, &mappings);
    let pt2 = part_2(&initial_values, &mappings);
    print_results(2024, 24, pt1, pt2, Some(start))
}

fn part_1(
    initial_values: &HashMap<&str, bool>,
    mappings: &HashMap<&str, (&str, &str, Gate)>,
) -> usize {
    let mut cache = HashMap::new();
    mappings
        .keys()
        .filter(|k| k.starts_with('z'))
        .sorted_unstable()
        .rev()
        .map(|k| wire_value(&mut cache, initial_values, mappings, 0, k))
        .fold_options(0, |acc, bit| (acc << 1) | (if bit { 1 } else { 0 }))
        .expect("no solution")
}

fn part_2(
    initial_values: &HashMap<&str, bool>,
    mappings: &HashMap<&str, (&str, &str, Gate)>,
) -> String {
    mappings
        .keys()
        .copied()
        .combinations(8)
        .map(|gates| {
            vec![
                [gates[0], gates[1]],
                [gates[2], gates[3]],
                [gates[4], gates[5]],
                [gates[6], gates[7]],
            ]
        })
        .par_bridge()
        .find_any(|swaps| -> bool {
            let mut cache = HashMap::new();
            let mut swapped_mappings = mappings.clone();
            for swap in swaps {
                let a = swap[0];
                let b = swap[1];
                let a_val = *swapped_mappings.get(a).unwrap();
                let b_val = *swapped_mappings.get(b).unwrap();
                swapped_mappings.insert(a, b_val);
                swapped_mappings.insert(b, a_val);
            }
            let Some(x) = swapped_mappings
                .keys()
                .filter(|k| k.starts_with('x'))
                .sorted_unstable()
                .rev()
                .map(|k| wire_value(&mut cache, initial_values, &swapped_mappings, 0, k))
                .fold_options(0, |acc, bit| (acc << 1) | (if bit { 1 } else { 0 }))
            else {
                return false;
            };
            let Some(y) = swapped_mappings
                .keys()
                .filter(|k| k.starts_with('y'))
                .sorted_unstable()
                .rev()
                .map(|k| wire_value(&mut cache, initial_values, &swapped_mappings, 0, k))
                .fold_options(0, |acc, bit| (acc << 1) | (if bit { 1 } else { 0 }))
            else {
                return false;
            };
            let Some(z) = swapped_mappings
                .keys()
                .filter(|k| k.starts_with('z'))
                .sorted_unstable()
                .rev()
                .map(|k| wire_value(&mut cache, initial_values, &swapped_mappings, 0, k))
                .fold_options(0, |acc, bit| (acc << 1) | (if bit { 1 } else { 0 }))
            else {
                return false;
            };
            x + y == z
        })
        .expect("no solution found")
        .into_iter()
        .flatten()
        .sorted_unstable()
        .join(",")
}

fn wire_value<'a>(
    cache: &mut HashMap<&'a str, bool>,
    initial_values: &HashMap<&'a str, bool>,
    mappings: &HashMap<&'a str, (&'a str, &'a str, Gate)>,
    depth: usize,
    key: &'a str,
) -> Option<bool> {
    if depth > 100 {
        return None;
    }

    if let Some(val) = cache.get(key) {
        return Some(*val);
    }

    if let Some(val) = initial_values.get(key) {
        return Some(*val);
    }

    let Some((a, b, gate)) = mappings.get(key) else {
        panic!("wire not found")
    };

    let a_val = wire_value(cache, initial_values, mappings, depth + 1, a)?;
    let b_val = wire_value(cache, initial_values, mappings, depth + 1, b)?;
    let rv = match gate {
        Gate::And => a_val && b_val,
        Gate::Or => a_val || b_val,
        Gate::Xor => a_val ^ b_val,
    };
    cache.insert(key, rv);
    Some(rv)
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
