use clap::Parser;
use itertools::Itertools;
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

    let solvers: HashMap<u32, Vec<fn()>> = HashMap::from([
        (
            2015,
            vec![
                advent_of_code::y2015::day_01::solve,
                advent_of_code::y2015::day_02::solve,
                advent_of_code::y2015::day_03::solve,
            ],
        ),
        (
            2022,
            vec![
                advent_of_code::y2022::day01::solve,
                advent_of_code::y2022::day02::solve,
                advent_of_code::y2022::day03::solve,
                advent_of_code::y2022::day04::solve,
                advent_of_code::y2022::day05::solve,
                advent_of_code::y2022::day06::solve,
                advent_of_code::y2022::day07::solve,
                advent_of_code::y2022::day08::solve,
                advent_of_code::y2022::day09::solve,
                advent_of_code::y2022::day10::solve,
                advent_of_code::y2022::day11::solve,
                advent_of_code::y2022::day12::solve,
                advent_of_code::y2022::day13::solve,
                advent_of_code::y2022::day14::solve,
                advent_of_code::y2022::day15::solve,
                advent_of_code::y2022::day16::solve,
                advent_of_code::y2022::day17::solve,
                advent_of_code::y2022::day18::solve,
                advent_of_code::y2022::day19::solve,
                advent_of_code::y2022::day20::solve,
                advent_of_code::y2022::day21::solve,
                advent_of_code::y2022::day22::solve,
                advent_of_code::y2022::day23::solve,
                advent_of_code::y2022::day24::solve,
                advent_of_code::y2022::day25::solve,
            ],
        ),
        (
            2023,
            vec![
                advent_of_code::y2023::day_01::solve,
                advent_of_code::y2023::day_02::solve,
                advent_of_code::y2023::day_03::solve,
                advent_of_code::y2023::day_04::solve,
                advent_of_code::y2023::day_05::solve,
                advent_of_code::y2023::day_06::solve,
                advent_of_code::y2023::day_07::solve,
                advent_of_code::y2023::day_08::solve,
                advent_of_code::y2023::day_09::solve,
                advent_of_code::y2023::day_10::solve,
            ],
        ),
        (
            2024,
            vec![
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
                advent_of_code::y2024::day19::solve,
                advent_of_code::y2024::day20::solve,
                advent_of_code::y2024::day21::solve,
                advent_of_code::y2024::day22::solve,
            ],
        ),
    ]);

    match (args.year, args.days) {
        (None, _) => solvers
            .iter()
            .sorted()
            .flat_map(|(_, ss)| ss)
            .for_each(|s| {
                s();
            }),
        (Some(year), days) if days.is_empty() => solvers
            .get(&year)
            .expect("year not found")
            .iter()
            .for_each(|s| {
                s();
            }),
        (Some(year), days) => {
            let ss = solvers.get(&year).expect("year not found");
            assert!(!days.contains(&0), "can't run day 0");
            assert!(*days.iter().max().unwrap() <= ss.len(), "day not found");
            days.iter().sorted().for_each(|d| {
                ss[*d - 1]();
            });
        }
    }
}
