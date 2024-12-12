use std::collections::HashMap;

fn main() {
    common::run_with_parser(
        |text| parse_input(&text),
        |numbers| simulate(numbers, 25),
        |numbers| simulate(numbers, 75),
    );
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn simulate(input: &[u64], steps: u32) -> usize {
    let mut previous: HashMap<u64, usize> = HashMap::new();
    let mut input = input.to_vec();
    input.sort();
    let mut now: HashMap<u64, usize> = input
        .chunk_by(|a, b| a == b)
        .map(|chunk| (chunk[0], chunk.len()))
        .collect();

    for (number, &occurences) in now.iter() {
        for _ in 0..occurences {
            eprint!("{number} ");
        }
    }
    eprintln!();

    for _ in 0..steps {
        (previous, now) = (now, previous);

        assert_ne!(previous.len(), 0);
        assert_eq!(now.len(), 0);

        for (number, occurences) in previous.drain() {
            let (a, b) = step_number(number);
            *now.entry(a).or_insert(0) += occurences;
            if let Some(b) = b {
                *now.entry(b).or_insert(0) += occurences;
            }
        }
    }

    now.values().sum()
}

fn step_number(number: u64) -> (u64, Option<u64>) {
    if number == 0 {
        return (1, None);
    }

    let mut power_10_n = 10;
    let mut power_10_2n = 100;
    let mut power_10_2n_minus_1 = 10;
    loop {
        if power_10_2n <= number {
            power_10_n *= 10;
            power_10_2n *= 100;
            power_10_2n_minus_1 *= 100;
            continue;
        }

        if power_10_2n_minus_1 <= number {
            return (number / power_10_n, Some(number % power_10_n));
        }

        return (number * 2024, None);
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, simulate};

    const SMALL_INPUT: &str = "125 17";

    #[test]
    fn test_simulate() {
        let input = parse_input(SMALL_INPUT);
        assert_eq!(simulate(&input, 25), 55312);
    }
}
