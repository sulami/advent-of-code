use crate::print_results;
use itertools::{repeat_n, Itertools};
use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;
use std::iter::repeat;
use std::ops::Add;
use std::time::Instant;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/15");
    let ingredients = parse_ingredients(input);
    let pt1 = one_hundred_spoons(&ingredients);
    let pt2 = five_hundred_calories(&ingredients);
    print_results(2015, 15, pt1, pt2, Some(start));
}

fn one_hundred_spoons(ingredients: &[Ingredient]) -> i32 {
    let mut queue: BinaryHeap<(Ingredient, Reverse<usize>)> =
        BinaryHeap::from_iter(ingredients.iter().copied().zip(repeat(Reverse(1))));

    while let Some((mix, Reverse(spoons))) = queue.pop() {
        if spoons == 100 {
            return mix.score();
        }
        for option in ingredients {
            queue.push((mix + *option, Reverse(spoons + 1)));
        }
    }

    panic!("mo mix found?!");
}

fn five_hundred_calories(ingredients: &[Ingredient]) -> i32 {
    repeat_n(0..=100, ingredients.len())
        .multi_cartesian_product()
        .filter(|counts| counts.iter().sum::<usize>() == 100)
        .map(|counts| {
            let mut mix = Ingredient::default();
            for (idx, count) in counts.iter().enumerate() {
                mix = repeat_n(ingredients[idx], *count).fold(mix, Add::add);
            }
            mix
        })
        .filter(|mix| mix.calories == 500)
        .map(|mix| mix.score())
        .max()
        .expect("no mix found")
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn score(&self) -> i32 {
        self.capacity.max(0) * self.durability.max(0) * self.flavor.max(0) * self.texture.max(0)
    }
}

impl Add for Ingredient {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            capacity: self.capacity + rhs.capacity,
            durability: self.durability + rhs.durability,
            flavor: self.flavor + rhs.flavor,
            texture: self.texture + rhs.texture,
            calories: self.calories + rhs.calories,
        }
    }
}

impl Ord for Ingredient {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score().cmp(&other.score())
    }
}

impl PartialOrd for Ingredient {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_ingredients(s: &str) -> Vec<Ingredient> {
    s.lines()
        .map(|l| {
            let words = l.split_whitespace().collect_vec();
            Ingredient {
                capacity: words[2].strip_suffix(',').unwrap().parse().unwrap(),
                durability: words[4].strip_suffix(',').unwrap().parse().unwrap(),
                flavor: words[6].strip_suffix(',').unwrap().parse().unwrap(),
                texture: words[8].strip_suffix(',').unwrap().parse().unwrap(),
                calories: words[10].parse().unwrap(),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn test_parser() {
        assert_eq!(
            parse_ingredients(INPUT),
            vec![
                Ingredient {
                    capacity: -1,
                    durability: -2,
                    flavor: 6,
                    texture: 3,
                    calories: 8
                },
                Ingredient {
                    capacity: 2,
                    durability: 3,
                    flavor: -2,
                    texture: -1,
                    calories: 3
                }
            ]
        );
    }

    #[test]
    fn test_one_hundred_spoons() {
        assert_eq!(one_hundred_spoons(&parse_ingredients(INPUT)), 62842880);
    }

    #[test]
    fn test_five_hundred_calories() {
        assert_eq!(five_hundred_calories(&parse_ingredients(INPUT)), 57600000);
    }
}
