use crate::coordinate_3d::Coordinate;
use ahash::HashMap;
use float_ord::FloatOrd;
use itertools::Itertools;

crate::solve!("08");

type Parsed = Vec<Coordinate>;

fn parse(input: &str) -> Parsed {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            let xs = l
                .split(',')
                .map(|n| n.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            (xs[0], xs[1], xs[2]).into()
        })
        .collect::<Vec<_>>()
}

fn part_1(x: &Parsed) -> usize {
    let mut nets = HashMap::from_iter(x.iter().zip(0..));
    let mut distances = x
        .iter()
        .tuple_combinations()
        .sorted_unstable_by_key(|(a, b)| FloatOrd(a.distance(**b)))
        .rev()
        .collect_vec();

    #[cfg(test)]
    let iterations = 10;
    #[cfg(not(test))]
    let iterations = 1000;

    for _ in 0..iterations {
        // Find the closest pair of points that are not yet connected.
        let (a, b) = distances.pop().unwrap();
        if nets[a] == nets[b] {
            continue;
        }

        // Merge the two nets.
        let host_net = nets[a];
        let child_net = nets[b];
        nets.iter_mut().for_each(|(_, n)| {
            if *n == child_net {
                *n = host_net;
            }
        });
    }

    nets.values()
        .counts()
        .values()
        .sorted_unstable()
        .rev()
        .take(3)
        .product::<usize>()
}

fn part_2(x: &Parsed) -> isize {
    let mut nets = HashMap::from_iter(x.iter().zip(0..));
    let mut connections = 0;
    let mut distances = x
        .iter()
        .tuple_combinations()
        .sorted_unstable_by_key(|(a, b)| FloatOrd(a.distance(**b)))
        .rev()
        .collect_vec();

    while connections < x.len() - 2 {
        // Find the closest pair of points that are not yet connected.
        let (a, b) = loop {
            let (a, b) = distances.pop().unwrap();
            if nets[a] != nets[b] {
                break (a, b);
            }
        };

        // Merge the two nets.
        let host_net = nets[a];
        let child_net = nets[b];
        nets.iter_mut().for_each(|(_, n)| {
            if *n == child_net {
                *n = host_net;
            }
        });

        connections += 1;
    }

    // Find the closest pair of points that are not yet connected.
    let final_pair = loop {
        let (a, b) = distances.pop().unwrap();
        if nets[a] != nets[b] {
            break (a, b);
        }
    };

    final_pair.0.x * final_pair.1.x
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&parse(INPUT)), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&parse(INPUT)), 25272);
    }
}
