use crate::print_results;
use ahash::HashMap;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/16");
    let aunts = parse_aunts(input);
    let search_key = HashMap::from_iter([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
    let pt1 = aunts
        .iter()
        .position(|aunt| {
            search_key
                .iter()
                .all(|(k, v)| aunt.get(k).is_none_or(|x| x == v))
        })
        .expect("no aunt found")
        + 1;
    let pt2 = aunts
        .iter()
        .position(|aunt| {
            search_key.iter().all(|(k, v)| {
                aunt.get(k).is_none_or(|x| match *k {
                    "cats" | "trees" => x > v,
                    "pomeranians" | "goldfish" => x < v,
                    _ => x == v,
                })
            })
        })
        .expect("no aunt found")
        + 1;
    print_results(2015, 16, pt1, pt2, Some(start));
}

fn parse_aunts(s: &str) -> Vec<HashMap<&str, u32>> {
    s.lines()
        .map(|l| {
            l.split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|p| p.split_once(": ").unwrap())
                .map(|(k, v)| (k, v.parse().unwrap()))
                .collect()
        })
        .collect()
}
