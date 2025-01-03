use crate::print_results;
use ahash::HashMap;
use itertools::Itertools;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/09");
    let map = parse(input);
    let pt1 = shortest_route(&map);
    let pt2 = longest_route(&map);
    print_results(2015, 9, pt1, pt2, Some(start));
}

type AdjacencyMap<'a> = HashMap<&'a str, HashMap<&'a str, usize>>;

fn shortest_route(map: &AdjacencyMap) -> usize {
    fn search<'a>(map: &AdjacencyMap<'a>, distance: usize, route: Vec<&'a str>) -> usize {
        if route.len() == map.len() {
            distance
        } else {
            map[route.last().unwrap()]
                .iter()
                .filter(|(dest, _)| !route.contains(dest))
                .map(|(dest, dist)| {
                    search(map, distance + *dist, {
                        let mut r = route.clone();
                        r.push(dest);
                        r
                    })
                })
                .min()
                .unwrap()
        }
    }

    map.keys()
        .map(|from| search(map, 0, vec![*from]))
        .min()
        .unwrap()
}

fn longest_route(map: &AdjacencyMap) -> usize {
    fn search<'a>(map: &AdjacencyMap<'a>, distance: usize, route: Vec<&'a str>) -> usize {
        if route.len() == map.len() {
            distance
        } else {
            map[route.last().unwrap()]
                .iter()
                .filter(|(dest, _)| !route.contains(dest))
                .map(|(dest, dist)| {
                    search(map, distance + *dist, {
                        let mut r = route.clone();
                        r.push(dest);
                        r
                    })
                })
                .max()
                .unwrap()
        }
    }

    map.keys()
        .map(|from| search(map, 0, vec![*from]))
        .max()
        .unwrap()
}

fn parse(s: &str) -> AdjacencyMap {
    let mut rv = HashMap::default();
    s.lines().for_each(|line| {
        let parts = line.split_whitespace().collect_vec();
        rv.entry(parts[0])
            .or_insert_with(HashMap::default)
            .insert(parts[2], parts[4].parse().expect("invalid distance"));
        rv.entry(parts[2])
            .or_insert_with(HashMap::default)
            .insert(parts[0], parts[4].parse().expect("invalid distance"));
    });
    rv
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn test_part_1() {
        assert_eq!(shortest_route(&parse(INPUT)), 605);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(longest_route(&parse(INPUT)), 982);
    }
}
