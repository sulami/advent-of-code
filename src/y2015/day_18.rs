use crate::{coordinate::Coordinate, print_results};
use ahash::HashMap;
use std::{iter::successors, time::Instant};

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/18");
    let lights = Coordinate::parse_grid(input, |c| Some(c == '#'));
    let pt1 = successors(Some(lights.clone()), |prev| Some(life(prev)))
        .nth(100)
        .unwrap()
        .into_iter()
        .filter(|(_, on)| *on)
        .count();
    let pt2 = successors(Some(turn_on_corners(lights)), |prev| {
        Some(turn_on_corners(life(prev)))
    })
    .nth(100)
    .unwrap()
    .into_iter()
    .filter(|(_, on)| *on)
    .count();
    print_results(2015, 18, pt1, pt2, Some(start));
}

fn life(lights: &HashMap<Coordinate, bool>) -> HashMap<Coordinate, bool> {
    let mut rv = lights.to_owned();
    for (coord, already_on) in rv.iter_mut() {
        let neighbours_on = coord
            .diagonal_neighbours(0..100)
            .into_iter()
            .filter(|c| *lights.get(c).unwrap())
            .count();
        if *already_on && !(2..=3).contains(&neighbours_on) {
            *already_on = false;
        } else if !*already_on && neighbours_on == 3 {
            *already_on = true;
        }
    }
    rv
}

fn turn_on_corners(mut lights: HashMap<Coordinate, bool>) -> HashMap<Coordinate, bool> {
    [(0, 0), (0, 99), (99, 0), (99, 99)]
        .into_iter()
        .map(Coordinate::from)
        .for_each(|c| {
            *lights.get_mut(&c).unwrap() = true;
        });
    lights
}
