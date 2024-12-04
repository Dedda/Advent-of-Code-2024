use std::str::Chars;
use enum_iterator::{all, Sequence};

#[derive(Debug, Copy, Clone, Sequence, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    LeftDown,
    LeftUp,
    RightDown,
    RightUp,
}

impl Direction {

    fn step(&self, (x, y): (usize, usize)) -> (bool, (usize, usize)) {
        let new = match self {
            Direction::Left if x > 0 => (x - 1, y),
            Direction::Right => (x + 1, y),
            Direction::Up if y > 0 => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::LeftDown if x > 0 => (x - 1, y + 1),
            Direction::LeftUp if x > 0 && y > 0 => (x - 1, y - 1),
            Direction::RightDown => (x + 1, y + 1),
            Direction::RightUp if y > 0 => (x + 1, y - 1),
            _ => return (false, (x, y)),
        };
        (true, new)
    }

    fn origins(&self, line_lengths: &Vec<usize>) -> Vec<(usize, usize)> {
        if line_lengths.is_empty() {
            return vec![];
        }
        match self {
            Direction::Left => (0..line_lengths.len()).map(|y| (line_lengths[y] - 1, y)).collect(),
            Direction::Right => (0..line_lengths.len()).map(|y| (0, y)).collect(),
            Direction::Up => (0..*line_lengths.last().unwrap()).map(|x| (x, line_lengths.len() - 1)).collect(),
            Direction::Down => (0..*line_lengths.first().unwrap()).map(|x| (x, 0)).collect(),
            Direction::LeftDown => {
                let mut right = (0..line_lengths.len()).map(|y| (line_lengths[y] - 1, y)).collect::<Vec<_>>();
                let mut top = (0..*line_lengths.first().unwrap()).map(|x| (x, 0)).collect();
                right.append(&mut top);
                right.sort();
                right.dedup();
                right
            },
            Direction::LeftUp => {
                let mut right = (0..line_lengths.len()).map(|y| (line_lengths[y] - 1, y)).collect::<Vec<_>>();
                let mut bottom = (0..*line_lengths.last().unwrap()).map(|x| (x, line_lengths.len() - 1)).collect();
                right.append(&mut bottom);
                right.sort();
                right.dedup();
                right
            },
            Direction::RightDown => {
                let mut left = (0..line_lengths.len()).map(|y| (0, y)).collect::<Vec<_>>();
                let mut top = (0..*line_lengths.first().unwrap()).map(|x| (x, 0)).collect();
                left.append(&mut top);
                left.sort();
                left.dedup();
                left
            }
            Direction::RightUp => {
                let mut left = (0..line_lengths.len()).map(|y| (0, y)).collect::<Vec<_>>();
                let mut bottom = (0..*line_lengths.last().unwrap()).map(|x| (x, line_lengths.len() - 1)).collect();
                left.append(&mut bottom);
                left.sort();
                left.dedup();
                left
            }
        }
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(str::chars).map(Chars::collect::<Vec<_>>).collect()
}

fn find_needles(data: &Vec<Vec<char>>, needle: &str) -> Vec<((usize, usize), Direction)> {
    let needle = needle.chars().rev().collect::<String>();
    let line_lengths = data.iter().map(|line| line.len()).collect::<Vec<_>>();
    all::<Direction>().map(|direction| {
        direction.origins(&line_lengths).iter().map(|origin| {
            let mut position = *origin;
            let mut found = vec![];
            let mut buffer = String::new();
            loop {
                if data.len() > position.1 && line_lengths[position.1] > position.0 {
                    buffer.push(data[position.1][position.0]);
                    if buffer.ends_with(&needle) {
                        found.push(((position.1, position.0), direction));
                    }
                    if let (true, new_position) = direction.step(position.clone()) {
                        position = new_position;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            found
        })
            .flatten()
            .collect::<Vec<_>>()
    })
        .flatten()
        .collect::<Vec<_>>()
}

fn find_pattern(data: &Vec<Vec<char>>, needle: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut found = vec![];
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if matches_pattern(data, needle, (x, y)) {
                found.push((x, y));
            }
        }
    }
    found
}

fn matches_pattern(data: &Vec<Vec<char>>, needle: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    for ny in 0..needle.len() {
        if data.len() <= y + ny {
            return false;
        }
        for nx in 0..needle[ny].len() {
            if data[y + ny].len() <= x + nx {
                return false;
            }
            if needle[ny][nx] == ' ' {
                continue;
            }
            if needle[ny][nx] != data[y + ny][x + nx] {
                return false;
            }
        }
    }
    true
}

fn part_1(data: &Vec<Vec<char>>) -> usize {
    let keyword = "XMAS";
    find_needles(data, keyword).len()
}

fn part_2(data: &Vec<Vec<char>>) -> usize {
    let patterns = vec![
        "M M\n A \nS S", "M S\n A \nM S", "S M\n A \nS M", "S S\n A \nM M",
        // " M \nMAS\n S ", " M \nSAM\n S ", " S \nMAS\n M ", " S \nSAM\n "
    ];
    patterns.iter()
        .map(|pattern| find_pattern(data, &pattern.lines().map(|line| line.chars().collect::<Vec<_>>()).collect()).len())
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    let parsed = parse(input);
    println!("Part 1: {}", part_1(&parsed));
    println!("Part 2: {}", part_2(&parsed));
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2};

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        let parsed = parse(input);
        assert_eq!(18, part_1(&parsed));
        assert_eq!(9, part_2(&parsed));
    }
}