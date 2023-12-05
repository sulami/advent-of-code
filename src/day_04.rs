use arrayvec::ArrayVec;
use nom::{
    bytes::complete::tag,
    character::complete::{space0, u32},
};

const MAX_WINNING_NUMBERS: usize = 10;

pub fn solve() -> (u32, u32) {
    let input = include_str!("../inputs/day_04");

    let part_1 = input.lines().map(winnings).map(|n| 2_u32.pow(n - 1)).sum();

    let mut part_2 = 0;
    let mut extra_cards: ArrayVec<u32, MAX_WINNING_NUMBERS> = ArrayVec::new();

    for card in input.lines() {
        let wins = winnings(card);
        let duplicates = if !extra_cards.is_empty() {
            extra_cards.remove(0)
        } else {
            0
        };
        part_2 += 1 + duplicates;
        for i in 0..wins as usize {
            if i < extra_cards.len() {
                extra_cards[i] += 1 + duplicates;
            } else {
                extra_cards.push(1 + duplicates);
            }
        }
    }

    (part_1, part_2)
}

fn winnings(mut card: &str) -> u32 {
    card = &card[card.find(':').unwrap() + 2..];

    let mut winning_numbers: ArrayVec<u32, MAX_WINNING_NUMBERS> = ArrayVec::new();
    let mut past_bar = false;
    let mut score = 0;

    while !card.is_empty() {
        let (rest, _) = space0::<&str, ()>(card).unwrap();
        if let Ok((rest, num)) = u32::<&str, ()>(rest) {
            if past_bar {
                if winning_numbers.contains(&num) {
                    score += 1;
                }
            } else {
                winning_numbers.push(num);
            }
            card = rest;
            continue;
        } else {
            let (rest, _) = tag::<&str, &str, ()>("|")(rest).unwrap();
            past_bar = true;
            card = rest;
            continue;
        }
    }

    score
}
