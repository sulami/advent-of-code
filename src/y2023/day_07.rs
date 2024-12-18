use core::cmp::Ordering;

pub fn solve() {
    let input = include_str!("inputs/day_07");

    let mut cards: Vec<(&str, u32)> = Vec::new();
    input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .for_each(|(hand, bid)| cards.push((hand, bid.parse::<u32>().unwrap())));
    cards.sort_unstable_by(|(left, _), (right, _)| cmp_hands(left, right, false));

    let part_1: u32 = cards
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| (i as u32 + 1) * bid)
        .sum();

    cards.sort_unstable_by(|(left, _), (right, _)| cmp_hands(left, right, true));
    let part_2: u32 = cards
        .iter()
        .enumerate()
        .map(|(i, (_hand, bid))| (i as u32 + 1) * bid)
        .sum();

    println!("{}\n{}", part_1 as u64, part_2 as u64)
}

fn cmp_hands(left: &str, right: &str, jokers: bool) -> Ordering {
    match hand_type(left, jokers).cmp(&hand_type(right, jokers)) {
        Ordering::Equal => {
            let card_power = if jokers {
                "J23456789TQKA"
            } else {
                "23456789TJQKA"
            };
            for (l, r) in left.chars().zip(right.chars()) {
                if l == r {
                    continue;
                }
                return card_power
                    .find(l)
                    .unwrap()
                    .cmp(&card_power.find(r).unwrap());
            }
            Ordering::Equal
        }
        order => order,
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_type(hand: &str, jokers: bool) -> HandType {
    let cards: Vec<char> = Vec::from_iter(hand.chars());
    let (count, most) = of_same_kind(&cards, jokers, None);
    if count == 5 {
        return HandType::FiveOfAKind;
    }
    if count == 4 {
        return HandType::FourOfAKind;
    }
    if count == 3 {
        if of_same_kind(&cards, jokers, Some(most)).0 == 2 {
            return HandType::FullHouse;
        } else {
            return HandType::ThreeOfAKind;
        }
    }
    if count == 2 {
        if of_same_kind(&cards, jokers, Some(most)).0 == 2 {
            return HandType::TwoPair;
        } else {
            return HandType::OnePair;
        }
    }
    HandType::HighCard
}

/// Most occuring char in Vec, and its count. Optionally deny-list a
/// char. Horribly specific function. I'm just having a bad day and
/// want to get this puzzle done.
fn of_same_kind(coll: &Vec<char>, jokers: bool, avoid: Option<char>) -> (usize, char) {
    // This is just evil.
    if coll.iter().all(|c| *c == 'J') {
        return (5, 'J');
    }

    let mut max = 1;
    // Avoid defaulting to a Joker for best card for a hand like
    // J2345.
    let mut best = if jokers {
        *coll.iter().find(|c| **c != 'J').unwrap()
    } else {
        coll[0]
    };

    for i in 0..coll.len() {
        let this_char = coll[i];

        // Don't count cards from the first round of counting.
        if let Some(avoid) = avoid {
            if this_char == avoid {
                continue;
            }
        }

        // Count jokers separately.
        if jokers && this_char == 'J' {
            continue;
        }

        // Count how many of this card we have.
        let mut this_count = 1;
        for j in i + 1..coll.len() {
            if this_char == coll[j] {
                this_count += 1;
            }
        }

        if this_count > max {
            max = this_count;
            best = this_char;
        }
    }

    // Add jokers for the initial best count.
    if jokers && avoid.is_none() {
        max += coll.iter().filter(|c| **c == 'J').count();
    }

    (max, best)
}
