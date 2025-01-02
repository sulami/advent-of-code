use crate::print_results;
use ahash::{HashMap, HashMapExt, HashSet};
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
) -> u64 {
    let mut cache = HashMap::new();
    mappings
        .keys()
        .filter(|k| k.starts_with('z'))
        .sorted_unstable()
        .rev()
        .map(|k| wire_value(&mut cache, initial_values, mappings, 89, k))
        .fold_options(0, |acc, bit| (acc << 1) | (if bit { 1 } else { 0 }))
        .expect("no solution")
}

fn part_2(
    initial_values: &HashMap<&str, bool>,
    mappings: &HashMap<&str, (&str, &str, Gate)>,
) -> String {
    let swaps = find_swaps(
        initial_values,
        mappings.clone(),
        HashSet::default(),
        0,
        HashSet::default(),
    )
    .expect("no solution found");
    swaps.into_iter().flatten().sorted_unstable().join(",")
}

#[allow(clippy::unnecessary_to_owned)]
fn find_swaps(
    initial_values: &HashMap<&str, bool>,
    mappings: HashMap<&str, (&str, &str, Gate)>,
    swaps: HashSet<[String; 2]>,
    mut first_bad_bit: usize,
    mut good_gates: HashSet<String>,
) -> Option<HashSet<[String; 2]>> {
    let acceptance_test = |mappings: &HashMap<&str, (&str, &str, Gate)>| -> bool {
        [
            (0, 0),
            (1, 1),
            (12345, 45678),
            (9_123_456, 9_987_654),
            (1_000_000_000, 9_999_999_999),
            (2_123_456_789_120, 8_765_432_109_876),
            (4_306_139_814_720, 9_742_484_220_047),
        ]
        .into_iter()
        .all(|(a, b)| {
            let mut initial_values = initial_values.clone();
            for i in 0..=44 {
                *initial_values.get_mut(format!("x{i:02}").as_str()).unwrap() = (a >> i) & 1 == 1;
                *initial_values.get_mut(format!("y{i:02}").as_str()).unwrap() = (b >> i) & 1 == 1;
            }
            let (x, y, z) = get_numbers(&initial_values, mappings).expect("invalid swaps");
            assert_eq!(x, a, "bad initial values patching");
            assert_eq!(y, b, "bad initial values patching");
            z == x + y
        })
    };

    if swaps.len() == 4 {
        let (x, y, z) = get_numbers(initial_values, &mappings)?;
        return if x + y == z && acceptance_test(&mappings) {
            Some(swaps)
        } else {
            None
        };
    }

    first_bad_bit = (first_bad_bit..=45)
        .find(|&bit| {
            if check_bit(bit, initial_values, &mappings) {
                good_gates
                    .extend(wire_gates(initial_values, &mappings, format!("z{bit:02}")).unwrap());
                false
            } else {
                true
            }
        })
        .expect("no bad bit");

    mappings
        .keys()
        .filter(|gate| !good_gates.contains(&gate.to_string()))
        .filter(|gate| !swaps.iter().flatten().contains(&gate.to_string()))
        .copied()
        .tuple_combinations()
        .par_bridge()
        .filter_map(|(a, b)| {
            let mut new_mappings = mappings.clone();
            let a_val = new_mappings[a];
            let b_val = new_mappings[b];
            new_mappings.insert(a, b_val);
            new_mappings.insert(b, a_val);

            if check_bit(first_bad_bit, initial_values, &new_mappings) {
                let mut new_good_gates = good_gates.clone();
                new_good_gates.extend(
                    wire_gates(
                        initial_values,
                        &new_mappings,
                        format!("z{first_bad_bit:02}"),
                    )
                    .unwrap(),
                );
                let new_first_bad_bit = first_bad_bit + 1;
                let mut new_swaps = swaps.clone();
                new_swaps.insert([a.to_string(), b.to_string()]);
                return find_swaps(
                    initial_values,
                    new_mappings,
                    new_swaps,
                    new_first_bad_bit,
                    new_good_gates,
                );
            }
            None
        })
        .find_any(|_| true)
}

fn check_bit(
    bit: usize,
    initial_values: &HashMap<&str, bool>,
    mappings: &HashMap<&str, (&str, &str, Gate)>,
) -> bool {
    let mut sources_cache = HashMap::default();
    let target_sources = (0..=bit.min(44))
        .flat_map(|b| [format!("x{b:02}"), format!("y{b:02}")])
        .collect::<HashSet<_>>();
    let Some(bit_sources) = wire_sources(
        &mut sources_cache,
        initial_values,
        mappings,
        100,
        format!("z{bit:02}"),
    ) else {
        return false;
    };

    if bit_sources != target_sources {
        return false;
    }
    if bit <= 44 {
        let next_target_sources = (0..=(bit + 1).min(44))
            .flat_map(|b| [format!("x{b:02}"), format!("y{b:02}")])
            .collect::<HashSet<_>>();
        if wire_sources(
            &mut sources_cache,
            initial_values,
            mappings,
            100,
            format!("z{:02}", bit + 1),
        )
        .is_none_or(|next_bit_sources| next_bit_sources != next_target_sources)
        {
            return false;
        }
    }

    std::iter::repeat_n([true, false], 4)
        .multi_cartesian_product()
        .all(|inputs| {
            let [a, b, c, d] = inputs[0..4] else {
                unreachable!("bad cartesian product")
            };
            let mut initial_values = initial_values.clone();
            for i in 0..=44 {
                *initial_values.get_mut(format!("x{i:02}").as_str()).unwrap() =
                    a && i == bit || c && i == bit + 1;
                *initial_values.get_mut(format!("y{i:02}").as_str()).unwrap() =
                    b && i == bit || d && i == bit + 1;
            }

            let Some(this_result) = wire_value(
                &mut HashMap::default(),
                &initial_values,
                mappings,
                89,
                &format!("z{:02}", bit),
            ) else {
                return false;
            };

            let Some(carry_result) = wire_value(
                &mut HashMap::default(),
                &initial_values,
                mappings,
                89,
                &format!("z{:02}", bit + 1),
            ) else {
                return false;
            };

            this_result == (a ^ b) && (carry_result == (a && b) ^ (c ^ d) || bit >= 44)
        })
}

/// Returns both inputs and the output as u64, parsed from bits.
fn get_numbers(
    initial_values: &HashMap<&str, bool>,
    mappings: &HashMap<&str, (&str, &str, Gate)>,
) -> Option<(u64, u64, u64)> {
    let mut cache = HashMap::new();
    let mut get_number = |register: char| -> Option<u64> {
        mappings
            .keys()
            .chain(initial_values.keys())
            .filter(|k| k.starts_with(register) && k.chars().skip(1).all(|c| c.is_ascii_digit()))
            .sorted_unstable()
            .rev()
            .map(|k| wire_value(&mut cache, initial_values, mappings, 100, k))
            .fold_options(0u64, |acc, bit| (acc << 1) | (if bit { 1 } else { 0 }))
    };
    let (x, y, z) = (get_number('x')?, get_number('y')?, get_number('z')?);
    Some((x, y, z))
}

/// Returns the names of all gates involved in producing a value for a given wire.
fn wire_gates<'a>(
    initial_values: &HashMap<&'a str, bool>,
    mappings: &HashMap<&'a str, (&'a str, &'a str, Gate)>,
    key: String,
) -> Option<HashSet<String>> {
    if initial_values.contains_key(key.as_str()) {
        return Some(HashSet::default());
    }

    let (a, b, _gate) = mappings.get(key.as_str()).expect("wire not found");
    let mut gates = HashSet::from_iter([key]);
    gates.extend(wire_gates(initial_values, mappings, a.to_string())?);
    gates.extend(wire_gates(initial_values, mappings, b.to_string())?);
    Some(gates)
}

/// Returns the names of all sources involved in producing a value for a given wire.
fn wire_sources<'a>(
    cache: &mut HashMap<String, HashSet<String>>,
    initial_values: &HashMap<&'a str, bool>,
    mappings: &HashMap<&'a str, (&'a str, &'a str, Gate)>,
    depth: usize,
    key: String,
) -> Option<HashSet<String>> {
    if depth == 0 {
        return None;
    }

    if let Some(val) = cache.get(&key) {
        return Some(val.clone());
    }

    if initial_values.contains_key(key.as_str()) {
        return Some(HashSet::from_iter([key]));
    }

    let (a, b, _gate) = mappings.get(key.as_str()).expect("wire not found");
    let mut sources = HashSet::default();
    sources.extend(wire_sources(
        cache,
        initial_values,
        mappings,
        depth - 1,
        a.to_string(),
    )?);
    sources.extend(wire_sources(
        cache,
        initial_values,
        mappings,
        depth - 1,
        b.to_string(),
    )?);
    cache.insert(key.clone(), sources.clone());
    Some(sources)
}

/// Returns the value of a given wire, with memoization.
fn wire_value<'a>(
    cache: &mut HashMap<&'a str, bool>,
    initial_values: &HashMap<&'a str, bool>,
    mappings: &HashMap<&'a str, (&'a str, &'a str, Gate)>,
    depth: usize,
    key: &'a str,
) -> Option<bool> {
    if depth == 0 {
        return None;
    }

    if let Some(val) = cache.get(key) {
        return Some(*val);
    }

    if let Some(val) = initial_values.get(key) {
        return Some(*val);
    }

    let (a, b, gate) = mappings.get(key).expect("wire not found");
    let a_val = wire_value(cache, initial_values, mappings, depth - 1, a)?;
    let b_val = wire_value(cache, initial_values, mappings, depth - 1, b)?;
    let rv = match gate {
        Gate::And => a_val && b_val,
        Gate::Or => a_val || b_val,
        Gate::Xor => a_val ^ b_val,
    };
    cache.insert(key, rv);
    Some(rv)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Gate {
    And,
    Or,
    Xor,
}

type Parsed<'a> = (
    HashMap<&'a str, bool>,
    HashMap<&'a str, (&'a str, &'a str, Gate)>,
);

fn parse_input(s: &str) -> Parsed {
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
        assert_eq!(parse_input(input), (HashMap::new(), HashMap::new()));
    }
}
