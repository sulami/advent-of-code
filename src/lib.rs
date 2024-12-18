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
