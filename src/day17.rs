use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline, u128 as parse_u128};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;

super::solve!("17");

fn parse(input: &str) -> Computer {
    parse_computer(input).expect("invalid computer").1
}

fn parse_computer(s: &str) -> IResult<&str, Computer> {
    let (s, a) = preceded(tag("Register A: "), parse_u128)(s)?;
    let (s, _) = newline(s)?;
    let (s, b) = preceded(tag("Register B: "), parse_u128)(s)?;
    let (s, _) = newline(s)?;
    let (s, c) = preceded(tag("Register C: "), parse_u128)(s)?;
    let (s, _) = newline(s)?;
    let (s, _) = newline(s)?;
    let (s, program) = preceded(tag("Program: "), separated_list1(char(','), parse_u128))(s)?;

    Ok((
        s,
        Computer {
            a,
            b,
            c,
            program,
            ptr: 0,
            output: vec![],
        },
    ))
}

fn part_1(computer: &Computer) -> String {
    let mut computer = computer.to_owned();
    computer.run();
    computer.output.iter().join(",")
}

fn part_2(computer: &Computer) -> u128 {
    let mut computer = computer.to_owned();
    let target = computer.program.clone();
    let mut a = 0;
    'outer: for digit in 0..computer.program.len() {
        for i in 0.. {
            computer.reset(i + a * 8);
            computer.run();
            if computer.output[..] == target[target.len() - digit - 1..] {
                a *= 8;
                a += i;
                continue 'outer;
            }
        }
    }
    a
}

#[derive(Clone, Debug)]
struct Computer {
    a: u128,
    b: u128,
    c: u128,
    ptr: usize,
    program: Vec<u128>,
    output: Vec<u128>,
}

impl Computer {
    #[inline(always)]
    fn arg(&self) -> u128 {
        unsafe { *self.program.get_unchecked(self.ptr + 1) }
    }

    #[inline(always)]
    fn combo_arg(&self) -> u128 {
        let val = unsafe { *self.program.get_unchecked(self.ptr + 1) };
        match val {
            0..=3 => val,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => unreachable!("invalid combo operand"),
            _ => unreachable!("more than 3 bits"),
        }
    }

    fn step(&mut self) {
        let opcode = unsafe { *self.program.get_unchecked(self.ptr) };
        match opcode {
            // adv
            0 => {
                self.a /= 2_u128
                    .checked_pow(self.combo_arg() as u32)
                    .expect("pow overflow")
            }
            // bxl
            1 => self.b ^= self.arg(),
            // bst
            2 => self.b = self.combo_arg() % 8,
            // jnz
            3 => {
                if self.a != 0 {
                    self.ptr = self.arg() as usize;
                }
            }
            // bxc
            4 => self.b ^= self.c,
            // out
            5 => {
                self.output.push(self.combo_arg() % 8);
            }
            // bdv
            6 => {
                self.b =
                    self.a / (2_u128.checked_pow(self.combo_arg() as u32)).expect("pow overflow")
            }
            // cdv
            7 => {
                self.c =
                    self.a / (2_u128.checked_pow(self.combo_arg() as u32)).expect("pow overflow")
            }
            _ => unreachable!("more than 3 bits"),
        }
        if opcode != 3 || self.a == 0 {
            self.ptr += 2;
        }
    }

    fn run(&mut self) {
        while self.ptr < self.program.len() {
            self.step();
        }
    }

    fn reset(&mut self, a: u128) {
        self.a = a;
        self.b = 0;
        self.c = 0;
        self.ptr = 0;
        self.output.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_part1() {
        assert_eq!(part_1(&parse(INPUT_1)), "4,6,3,5,6,3,5,2,1,0");
    }

    const INPUT_2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part2() {
        assert_eq!(part_2(&parse(INPUT_2)), 117440);
    }
}
