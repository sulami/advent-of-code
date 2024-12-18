use clap::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Year to run
    year: Option<u32>,
    /// Days to run
    days: Vec<usize>,
}

fn main() {
    let args = Args::parse();

    let solvers = HashMap::from([(
        2024,
        [
            advent_of_code::y2024::day01::solve,
            advent_of_code::y2024::day02::solve,
            advent_of_code::y2024::day03::solve,
            advent_of_code::y2024::day04::solve,
            advent_of_code::y2024::day05::solve,
            advent_of_code::y2024::day06::solve,
            advent_of_code::y2024::day07::solve,
            advent_of_code::y2024::day08::solve,
            advent_of_code::y2024::day09::solve,
            advent_of_code::y2024::day10::solve,
            advent_of_code::y2024::day11::solve,
            advent_of_code::y2024::day12::solve,
            advent_of_code::y2024::day13::solve,
            advent_of_code::y2024::day14::solve,
            advent_of_code::y2024::day15::solve,
            advent_of_code::y2024::day16::solve,
            advent_of_code::y2024::day17::solve,
            advent_of_code::y2024::day18::solve,
        ],
    )]);

    match (args.year, args.days) {
        (None, _) => solvers.iter().flat_map(|(_, ss)| ss).for_each(|s| s()),
        (Some(year), days) if days.is_empty() => solvers
            .get(&year)
            .expect("year not found")
            .iter()
            .for_each(|s| s()),
        (Some(year), days) => {
            let ss = solvers.get(&year).expect("year not found");
            assert!(!days.contains(&0), "can't run day 0");
            assert!(*days.iter().max().unwrap() <= ss.len(), "day not found");
            days.iter().for_each(|d| ss[*d - 1]());
        }
    }
}
