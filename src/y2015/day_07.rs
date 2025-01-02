use crate::print_results;
use ahash::HashMap;
use itertools::Itertools;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/07");
    let mut mappings = parse(input);
    let mut cache = HashMap::default();
    let pt1 = resolve(&mut cache, &mappings, "a");
    let mut cache = HashMap::default();
    mappings.insert("b", Gate::Just(Value::Signal(pt1)));
    let pt2 = resolve(&mut cache, &mappings, "a");
    print_results(2015, 7, pt1, pt2, Some(start));
}

fn resolve<'a>(
    cache: &mut HashMap<&'a str, u16>,
    mappings: &'a HashMap<&'a str, Gate>,
    wire: &'a str,
) -> u16 {
    if let Some(v) = cache.get(wire) {
        return *v;
    }

    let mut resolve_value = |v| match v {
        Value::Signal(v) => v,
        Value::Wire(v) => resolve(cache, mappings, v),
    };

    let rv = match mappings.get(wire) {
        Some(Gate::Just(v)) => resolve_value(*v),
        Some(Gate::And(a, b)) => resolve_value(*a) & resolve_value(*b),
        Some(Gate::Or(a, b)) => resolve_value(*a) | resolve_value(*b),
        Some(Gate::Not(v)) => !resolve_value(*v),
        Some(Gate::LShift(v, by)) => resolve_value(*v) << by,
        Some(Gate::RShift(v, by)) => resolve_value(*v) >> by,
        _ => panic!("wire not found: {wire}"),
    };
    cache.insert(wire, rv);
    rv
}

#[derive(Debug, Clone, Copy)]
enum Value<'a> {
    Wire(&'a str),
    Signal(u16),
}

#[derive(Debug, Clone, Copy)]
enum Gate<'a> {
    Just(Value<'a>),
    And(Value<'a>, Value<'a>),
    Or(Value<'a>, Value<'a>),
    Not(Value<'a>),
    LShift(Value<'a>, usize),
    RShift(Value<'a>, usize),
}

fn parse_value(s: &str) -> Value {
    s.parse::<u16>()
        .map(Value::Signal)
        .unwrap_or(Value::Wire(s))
}

fn parse(s: &str) -> HashMap<&str, Gate> {
    s.lines()
        .map(|l| {
            let parts = l.split_whitespace().collect_vec();
            match parts.len() {
                3 => (parts[2], Gate::Just(parse_value(parts[0]))),
                4 => (parts[3], Gate::Not(parse_value(parts[1]))),
                5 => (
                    parts[4],
                    match parts[1] {
                        "AND" => Gate::And(parse_value(parts[0]), parse_value(parts[2])),
                        "OR" => Gate::Or(parse_value(parts[0]), parse_value(parts[2])),
                        "LSHIFT" => Gate::LShift(
                            parse_value(parts[0]),
                            parts[2].parse::<usize>().expect("invalid lshift"),
                        ),
                        "RSHIFT" => Gate::RShift(
                            parse_value(parts[0]),
                            parts[2].parse::<usize>().expect("invalid rshift"),
                        ),
                        _ => panic!("invalid input"),
                    },
                ),
                _ => panic!("invalid input"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let mappings = parse(input);
        for (k, v) in [
            ("d", 72),
            ("e", 507),
            ("f", 492),
            ("g", 114),
            ("h", 65412),
            ("i", 65079),
            ("x", 123),
            ("y", 456),
        ] {
            assert_eq!(resolve(&mut HashMap::default(), &mappings, k), v);
        }
    }
}
