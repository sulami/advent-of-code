use rustc_hash::FxHashSet;

super::solve!("12");

type Plant = char;
type Plot = (usize, usize);
type Region = FxHashSet<Plot>;

fn parse(input: &str) -> Vec<Region> {
    let mut plots = FxHashSet::default();
    for (y, line) in input.lines().enumerate() {
        for (x, plant) in line.chars().enumerate() {
            plots.insert((plant, (x, y)));
        }
    }
    let mut regions = vec![];
    while !plots.is_empty() {
        let p = *plots.iter().next().unwrap();
        let plot = plots.take(&p).unwrap();
        let mut region = FxHashSet::from_iter([plot.1]);
        fill_region(plot, &mut plots, &mut region);
        regions.push(region);
    }
    regions
}

fn fill_region(
    (plant, (x, y)): (Plant, Plot),
    from: &mut FxHashSet<(Plant, Plot)>,
    to: &mut Region,
) {
    [
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ]
    .iter()
    .for_each(|(x, y)| {
        if to.contains(&(*x, *y)) {
            return;
        }
        if let Some(p) = from.take(&(plant, (*x, *y))) {
            to.insert(p.1);
            fill_region(p, from, to)
        }
    });
}

fn part_1(regions: &[Region]) -> usize {
    regions.iter().map(price).sum()
}

fn part_2(regions: &[Region]) -> usize {
    regions.iter().map(price_with_bulk_discount).sum()
}

fn price(region: &Region) -> usize {
    let area = region.len();
    let perimeter: usize = region
        .iter()
        .map(|&(x, y)| {
            [
                (x.wrapping_sub(1), y),
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ]
            .iter()
            .filter(|&neighbour| !region.contains(neighbour))
            .count()
        })
        .sum();
    area * perimeter
}

fn price_with_bulk_discount(region: &Region) -> usize {
    let area = region.len();
    let corners: usize = region
        .iter()
        .filter(|&plot| is_perimeter(plot, region))
        .map(|&plot| external_corners(plot, region))
        .sum();
    area * corners
}

/// True if this plot has a non-region plot anywhere in its eight surrounding spaces.
fn is_perimeter(&(x, y): &Plot, region: &Region) -> bool {
    [
        (x.wrapping_sub(1), y.wrapping_sub(1)),
        (x.wrapping_sub(1), y),
        (x.wrapping_sub(1), y + 1),
        (x, y + 1),
        (x, y.wrapping_sub(1)),
        (x + 1, y.wrapping_sub(1)),
        (x + 1, y),
        (x + 1, y + 1),
    ]
    .iter()
    .any(|neighbour| !region.contains(neighbour))
}

/// Counts the corners of this plot that are either convex or concave corners of the
/// entire region, by looking at two adjacent non-diagonal neighbours as well as the
/// diagonal one between them.
fn external_corners((x, y): Plot, region: &Region) -> usize {
    [
        (
            (x.wrapping_sub(1), y),
            (x, y.wrapping_sub(1)),
            (x.wrapping_sub(1), y.wrapping_sub(1)),
        ),
        (
            (x, y.wrapping_sub(1)),
            (x + 1, y),
            (x + 1, y.wrapping_sub(1)),
        ),
        ((x + 1, y), (x, y + 1), (x + 1, y + 1)),
        (
            (x, y + 1),
            (x.wrapping_sub(1), y),
            (x.wrapping_sub(1), y + 1),
        ),
    ]
    .iter()
    .filter(|(adj_a, adj_b, diag)| {
        let a = region.contains(adj_a);
        let b = region.contains(adj_b);
        let c = region.contains(diag);
        a && b && !c || !a && !b
    })
    .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(INPUT)), 1930);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&parse(INPUT)), 1206);
    }
}
