fn main() {
    let input = include_str!("input.txt");
    let parsed = parse(input);
    println!("part 1: {}", part_1(&parsed));
    println!("part 2: {}", part_2(&parsed));
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect()
}

fn part_1(input: &Vec<Vec<usize>>) -> usize {
    input.iter().filter(|r| is_safe(r)).count()
}

fn part_2(input: &Vec<Vec<usize>>) -> usize {
    input.iter().filter(|report| {
        if is_safe(report) {
            return true;
        }
        for i in 0..report.len() {
            let mut dampened = (*report).clone();
            dampened.remove(i);
            if is_safe(&dampened) {
                return true;
            }
        }
        false
    }).count()
}

fn is_safe(report: &Vec<usize>) -> bool {
    all_increasing(report) || all_decreasing(report)
}

fn all_increasing(report: &Vec<usize>) -> bool {
    let mut iter = report.iter();
    let first = iter.next().unwrap();
    iter.fold((first, true), |(last, val), current| {
        let valid = val && last < current && current.abs_diff(*last) <= 3;
        (current, valid)
    }).1
}

fn all_decreasing(report: &Vec<usize>) -> bool {
    let mut iter = report.iter();
    let first = iter.next().unwrap();
    iter.fold((first, true), |(last, val), current| {
        let valid = val && last > current && current.abs_diff(*last) <= 3;
        (current, valid)
    }).1
}

#[cfg(test)]
mod tests {
    use crate::{parse, part_1, part_2};

    #[test]
    fn example() {
        let input = include_str!("example.txt");
        let parsed = parse(input);
        assert_eq!(2, part_1(&parsed));
        assert_eq!(4, part_2(&parsed));
    }
}
