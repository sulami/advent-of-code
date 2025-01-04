use crate::print_results;
use ahash::HashMap;
use itertools::Itertools;
use std::iter;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/13");
    let map = build_map(input);
    let pt1 = optimal_arrangment(&map);
    let pt2_map = map
        .iter()
        .map(|(&k, v)| {
            let mut v = v.clone();
            v.insert("me", 0);
            (k, v)
        })
        .chain(iter::once(("me", map.keys().map(|&k| (k, 0)).collect())))
        .collect();
    let pt2 = optimal_arrangment(&pt2_map);
    print_results(2015, 13, pt1, pt2, Some(start));
}

fn optimal_arrangment(map: &HashMap<&str, HashMap<&str, i32>>) -> i32 {
    map.keys()
        .permutations(map.len())
        .map(|order| {
            order
                .iter()
                .circular_tuple_windows()
                .map(|(a, b)| map[*a][*b] + map[*b][*a])
                .sum()
        })
        .max()
        .expect("no solution")
}

fn build_map(s: &str) -> HashMap<&str, HashMap<&str, i32>> {
    let mut rv: HashMap<&str, HashMap<&str, i32>> = HashMap::default();
    s.lines().for_each(|line| {
        let words = line
            .strip_suffix('.')
            .expect("no full stop found")
            .split_whitespace()
            .collect_vec();
        let gain = if words[2] == "gain" { 1 } else { -1 };
        rv.entry(words[0]).or_default().insert(
            words[10],
            words[3].parse::<i32>().expect("invalid happiness") * gain,
        );
    });
    rv
}
