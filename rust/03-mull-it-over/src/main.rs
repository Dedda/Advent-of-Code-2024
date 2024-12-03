use regex::Regex;
use crate::Instruction::{Do, Dont, Mul};

#[derive(Debug)]
enum Instruction {
    Mul(usize, usize),
    Do,
    Dont,
}

fn main() {
    let input = include_str!("input.txt");
    let parsed = parse(input);
    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));
}

fn parse(input: &str) -> Vec<Vec<Instruction>> {
    let mul_pattern = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    input.lines().map(|line| {
        let mut instructions = vec![];
        for i in 0..line.len() {
            if line[i..].starts_with("do()") {
                instructions.push(Do);
                continue;
            } else if line[i..].starts_with("don't()") {
                instructions.push(Dont);
                continue;
            }
            let found = mul_pattern.captures(&line[i..]);
            if let Some(found) = found {
                let (_, [a, b]) = found.extract();
                instructions.push(Mul(a.parse().unwrap(), b.parse().unwrap()));
            }
        }
        instructions
    }).collect()
}

fn part_1(input: &Vec<Vec<Instruction>>) -> usize {
    input.iter()
        .flatten()
        .map(|i| match i {
            Mul(a, b ) => a * b,
            _ => 0,
        })
        .sum()
}

fn part_2(input: &Vec<Vec<Instruction>>) -> usize {
    input.iter()
        .flatten()
        .fold((true, 0), |(enabled, sum), instruction| {
            match instruction {
                Mul(a, b) => if enabled { (enabled, sum + a*b) } else { (enabled, sum) },
                Do => (true, sum),
                Dont => (false, sum),
            }
    }).1
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2};

    #[test]
    fn example_1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let parsed = parse(input);
        assert_eq!(161, part_1(&parsed));
    }

    #[test]
    fn example_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let parsed = parse(input);
        assert_eq!(48, part_2(&parsed));
    }
}