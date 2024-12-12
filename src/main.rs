mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

fn main() {
    day01::solve();
    day02::solve();
    day03::solve();
    day04::solve();
    day05::solve();
    day06::solve();
    day07::solve();
    day08::solve();
    day09::solve();
    day10::solve();
    day11::solve();
    day12::solve();
}

#[macro_export]
macro_rules! solve {
    ($day:expr) => {
        pub fn solve() {
            let input = parse(include_str!(concat!("inputs/", $day)));
            println!("Day {}:", $day);
            println!("  {:>15}", part_1(&input));
            println!("  {:>15}", part_2(&input));
        }
    };
}
