use std::time::Instant;
use crate::print_results;

pub fn solve() {
    let start = Instant::now();
    let input = include_str!("inputs/25");
    let keyings = parse_keyings(input);
    let pt1 = keyings.iter().filter(|r| r.kind == Kind::Lock).map(|lock|
        keyings.iter().filter(|r| r.kind == Kind::Key).filter(|key| {
            lock.keying.iter().zip(key.keying).map(|(l, k)| l + k).all(|x| x <= 7)
        }).count()
    ).sum::<usize>();
    print_results(2024, 25, pt1, 0, Some(start));
}

type Keying = [u32; 5];

#[derive(PartialEq)]
enum Kind {
    Lock, Key,
}

struct Record {
    kind: Kind,
    keying: Keying,
}

fn parse_keyings(s: &str) -> Vec<Record> {
    s.split("\n\n").map(|record| {
        let kind = if record.starts_with("#####") {
            Kind::Key
        } else {
            Kind::Lock
        };
        let mut keying = [0; 5];
        record.lines().for_each(|l| {
            l.char_indices().for_each(|(idx, c)| {
                if c == '#' {
                    keying[idx] += 1;
                }
            });
        });
        Record { kind, keying }
    }).collect()
}
