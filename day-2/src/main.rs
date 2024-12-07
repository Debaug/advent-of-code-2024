use common::input;
use itertools::Itertools;

struct Input {
    reports: Vec<Vec<i32>>,
}

fn main() {
    let input = parse_input(&input());

    // part 1
    println!("part 1: {}", part1(&input));

    // part 2
    println!("part 2: {}", part2(&input));
}

fn parse_input(input: &str) -> Input {
    let reports = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse().expect("failed to parse level"))
                .collect()
        })
        .collect();
    Input { reports }
}

fn part1(input: &Input) -> usize {
    input
        .reports
        .iter()
        .filter(|report| report_is_safe(report))
        .count()
}

fn part2(input: &Input) -> usize {
    input
        .reports
        .iter()
        .filter(|report| report_is_safe_with_skip(report))
        .count()
}

fn report_is_safe(report: &[i32]) -> bool {
    if report.len() <= 1 {
        return true;
    }

    let increasing = report[0] < report[1];
    for (&a, &b) in report.iter().tuple_windows() {
        let diff = b - a;
        let diff_is_good =
            (increasing && (1..=3).contains(&diff)) || (!increasing && (-3..=-1).contains(&diff));
        if !diff_is_good {
            return false;
        }
    }

    true
}

fn report_is_safe_with_skip(report: &[i32]) -> bool {
    if report_is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let report_without_i = report
            .iter()
            .enumerate()
            .filter_map(|(idx, &level)| (idx != i).then_some(level))
            .collect_vec();
        if report_is_safe(&report_without_i) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    const SMALL_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_parse_input() {
        let reports = parse_input(SMALL_INPUT).reports;
        assert_eq!(
            reports,
            &[
                &[7, 6, 4, 2, 1],
                &[1, 2, 7, 8, 9],
                &[9, 7, 6, 2, 1],
                &[1, 3, 2, 4, 5],
                &[8, 6, 4, 4, 1],
                &[1, 3, 6, 7, 9],
            ],
        )
    }

    #[test]
    fn test_part1() {
        let num_safe = part1(&parse_input(SMALL_INPUT));
        assert_eq!(num_safe, 2);
    }

    #[test]
    fn test_part2() {
        let num_safe = part2(&parse_input(SMALL_INPUT));
        assert_eq!(num_safe, 4);
    }
}
