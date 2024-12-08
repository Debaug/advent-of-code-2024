use std::{convert::Infallible, iter, str::FromStr};

fn main() {
    common::run_with_parser(|text| text.parse::<Input>().unwrap(), part1, part2);
}

#[derive(Debug)]
struct EquationTest {
    terms: Vec<u64>,
    result: u64,
}

#[derive(Debug)]
struct Input {
    equations: Vec<EquationTest>,
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

impl FromStr for Input {
    type Err = Infallible;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let equations = text
            .lines()
            .map(|line| {
                let mut parts = line.split(": ");

                let result = parts
                    .next()
                    .expect("invalid equation input")
                    .parse()
                    .expect("invalid number");

                let terms = parts
                    .next()
                    .expect("invalid equation input")
                    .split(' ')
                    .map(|word| word.parse().expect("invalid number"))
                    .collect();

                EquationTest { result, terms }
            })
            .collect();
        Ok(Self { equations })
    }
}

fn is_valid_equation1(terms: &[u64], result: u64) -> bool {
    if terms.len() == 1 {
        return terms[0] == result;
    }

    let last = *terms.last().expect("empty equation");

    if last > result {
        return false;
    }

    if is_valid_equation1(&terms[..(terms.len() - 1)], result - last) {
        return true;
    }

    if (result % last == 0) && is_valid_equation1(&terms[..(terms.len() - 1)], result / last) {
        return true;
    }

    false
}

fn is_valid_equation2(terms: &[u64], result: u64) -> bool {
    if terms.len() == 1 {
        return terms[0] == result;
    }

    let last = *terms.last().expect("empty equation");

    if last > result {
        return false;
    }

    if is_valid_equation2(&terms[..(terms.len() - 1)], result - last) {
        return true;
    }

    if (result % last == 0) && is_valid_equation2(&terms[..(terms.len() - 1)], result / last) {
        return true;
    }

    let mut powers_of_10 = iter::successors(Some(10), |&n| Some(n * 10));
    let next_power_of_10 = powers_of_10.find(|&n| n > last).unwrap();
    if (result % next_power_of_10 == last)
        && is_valid_equation2(&terms[..(terms.len() - 1)], result / next_power_of_10)
    {
        return true;
    }

    false
}

fn part1(input: &Input) -> u64 {
    input
        .equations
        .iter()
        .filter(|equation| is_valid_equation1(&equation.terms, equation.result))
        .map(|equation| equation.result)
        .sum()
}

fn part2(input: &Input) -> u64 {
    input
        .equations
        .iter()
        .filter(|equation| is_valid_equation2(&equation.terms, equation.result))
        .map(|equation| equation.result)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const SMALL_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        let input = SMALL_INPUT.parse().unwrap();
        assert_eq!(part1(&input), 3749);
    }

    #[test]
    fn test_part2() {
        let input = SMALL_INPUT.parse().unwrap();
        assert_eq!(part2(&input), 11387);
    }
}
