fn main() {
    let input = include_str!("input.txt");
    let sum = part_1(input);
    println!("Part 1: {}", sum);
    let sum = part_2(input);
    println!("Part 2: {}", sum);
}

fn part_1(input: &str) -> usize {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    left.iter().zip(right.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
}

fn part_2(input: &str) -> usize {
    let (left, right) = parse_input(input);
    left.iter()
        .map(|id|
            id * right.iter()
                .filter(|rid| **rid == *id)
                .count())
        .sum()
}

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (left, right): (Vec<_>, Vec<_>) = input.lines().map(|line| {
        let mut parts = line.split(' ').filter(|part| !part.is_empty());
        let left = parts.next().unwrap().parse::<usize>().unwrap();
        let right = parts.next().unwrap().parse::<usize>().unwrap();
        return (left, right);
    }).unzip();
    (left, right)
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_part_1() {
        let input = include_str!("example.txt");
        let sum = part_1(input);
        assert_eq!(11, sum);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("example.txt");
        let sum = part_2(input);
        assert_eq!(31, sum);
    }
}