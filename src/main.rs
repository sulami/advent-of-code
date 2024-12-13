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
            let start = std::time::Instant::now();
            let input = parse(include_str!(concat!("inputs/", $day)));
            let a = part_1(&input);
            let b = part_2(&input);
            println!("Day {}:  ({:>3} ms)", $day, start.elapsed().as_millis());
            println!("  {:>15}", a);
            println!("  {:>15}", b);
        }
    };
}
