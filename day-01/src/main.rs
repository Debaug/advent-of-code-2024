use std::collections::HashMap;

fn main() {
    common::run(part1, part2);
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines().filter(|s| !s.is_empty()) {
        let mut numbers = line.split(' ').filter(|s| !s.is_empty());
        left.push(
            numbers
                .next()
                .expect("missing number")
                .parse()
                .expect("invalid number"),
        );
        right.push(
            numbers
                .next()
                .expect("missing second number")
                .parse()
                .expect("invalid number"),
        );
    }
    (left, right)
}

fn part1(input: &str) -> u32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();
    left.iter().zip(&right).map(|(&l, &r)| l.abs_diff(r)).sum()
}

fn part2(input: &str) -> i32 {
    let (left, right) = parse_input(input);
    let mut occurences_in_right: HashMap<i32, i32> = HashMap::new();
    for &n in &right {
        *occurences_in_right.entry(n).or_insert(0) += 1;
    }
    let mut score = 0;
    for &n in &left {
        score += n * occurences_in_right.get(&n).copied().unwrap_or(0);
    }
    score
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    const SMALL_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_parse_input() {
        let (left, right) = parse_input(SMALL_INPUT);
        assert_eq!(left, &[3, 4, 2, 1, 3, 3]);
        assert_eq!(right, &[4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_part1() {
        let distance = part1(SMALL_INPUT);
        assert_eq!(distance, 11);
    }

    #[test]
    fn test_part2() {
        let score = part2(SMALL_INPUT);
        assert_eq!(score, 31);
    }
}
