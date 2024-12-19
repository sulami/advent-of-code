use std::{fmt::Display, time::Instant};

pub mod y2015;
pub mod y2022;
pub mod y2023;
pub mod y2024;

#[macro_export]
macro_rules! solve {
    ($day:expr) => {
        pub fn solve() {
            let start = std::time::Instant::now();
            let input = parse(include_str!(concat!("inputs/", $day)));
            let a = part_1(&input);
            let b = part_2(&input);
            println!("Day {}:     ({:>3} ms)", $day, start.elapsed().as_millis());
            println!("{:>20}", a);
            println!("{:>20}", b);
        }
    };
}

/// Prints results in standardized way, optionally also capturing timing data.
pub fn print_results<A, B>(year: u32, day: u32, part_1: A, part_2: B, started_at: Option<Instant>)
where
    A: Display,
    B: Display,
{
    let timing = match started_at {
        Some(t) => &format!("({:>3} ms)", t.elapsed().as_millis()),
        None => "",
    };
    println!("{year}-{day:02}:      {timing}");
    println!("{part_1}");
    println!("{part_2}");
}
