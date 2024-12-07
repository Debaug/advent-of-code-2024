use std::{convert::Infallible, str::FromStr};

mod part1;
mod part2;

fn main() {
    common::run_with_parser(
        |text| text.parse::<Input>().unwrap(),
        part1::part1,
        part2::part2,
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

struct Input {
    rows: Vec<Vec<Option<Letter>>>,
}

impl FromStr for Input {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s
            .lines()
            .map(|line| line.chars().map(|c| c.try_into().ok()).collect())
            .collect();
        Ok(Self { rows })
    }
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

impl Input {
    fn get(&self, x: isize, y: isize) -> Option<Letter> {
        let Ok(x): Result<usize, _> = x.try_into() else {
            return None;
        };
        let Ok(y): Result<usize, _> = y.try_into() else {
            return None;
        };

        self.rows
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .flatten()
    }

    fn indexed_letters(&self) -> impl Iterator<Item = (isize, isize, Letter)> + '_ {
        self.rows.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, l)| Some((x as isize, y as isize, (*l)?)))
        })
    }
}

impl TryFrom<char> for Letter {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'X' => Ok(Self::X),
            'M' => Ok(Self::M),
            'A' => Ok(Self::A),
            'S' => Ok(Self::S),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Input, Letter};

    pub const SMALL_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn test_parse_input() {
        let tiny_input: &str = "XMAS
NO THING

MIXED";
        let input: Input = tiny_input.parse().unwrap();

        let x = Some(Letter::X);
        let m = Some(Letter::M);
        let a = Some(Letter::A);
        let s = Some(Letter::S);
        let n = None;

        let expected: &[&[Option<Letter>]] = &[&[x, m, a, s], &[n; 8], &[], &[m, n, x, n, n]];

        assert_eq!(input.rows, expected);
    }
}
