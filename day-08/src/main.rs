use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
};

use common::math::{Pos, Vect};

fn main() {
    common::run_with_parser(|text| text.parse::<Input>().unwrap(), part1, part2);
}

struct Input {
    antennas: HashMap<char, Vec<Pos>>,
    width: usize,
    height: usize,
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

impl FromStr for Input {
    type Err = Infallible;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();
        let mut width = 0;
        let mut height = 0;

        text.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                width = width.max(line.len());
                height += 1;
                line.chars().enumerate().map(move |(x, c)| (x, y, c))
            })
            .filter(|(.., c)| *c != '.')
            .for_each(|(x, y, c)| {
                assert!(
                    c.is_ascii_digit() || c.is_ascii_lowercase() || c.is_ascii_uppercase(),
                    "invalid frequency character"
                );
                antennas.entry(c).or_default().push(Pos {
                    x: x as isize,
                    y: y as isize,
                });
            });

        Ok(Self {
            antennas,
            width,
            height,
        })
    }
}

fn frequency_antinodes(antennas: &[Pos], width: usize, height: usize, out: &mut HashSet<Pos>) {
    for (i, &pos_a) in antennas.iter().enumerate() {
        for &pos_b in &antennas[(i + 1)..] {
            let diff = pos_a - pos_b;

            let antinode_a = pos_a + diff;
            if antinode_a.in_bounds(width, height) {
                out.insert(antinode_a);
            }

            let antinode_b = pos_b - diff;
            if antinode_b.in_bounds(width, height) {
                out.insert(antinode_b);
            }
        }
    }
}

fn part1(input: &Input) -> usize {
    let mut antinodes = HashSet::new();
    for antennas in input.antennas.values() {
        frequency_antinodes(antennas, input.width, input.height, &mut antinodes);
    }
    antinodes.len()
}

fn coprime_direction(a: Pos, b: Pos) -> Vect {
    let Vect { x, y } = b - a;
    for n in (2..=x.abs().min(y.abs())).rev() {
        if x % n == 0 && y % n == 0 {
            return Vect { x: x / n, y: y / n };
        }
    }
    Vect { x, y }
}

fn ray(a: Pos, direction: Vect) -> impl Iterator<Item = Pos> {
    (0..).map(move |t| a + t * direction)
}

fn antinodes_for_line2(a: Pos, b: Pos, width: usize, height: usize, out: &mut HashSet<Pos>) {
    let direction = coprime_direction(a, b);
    let before = ray(a, -direction).take_while(|pos| pos.in_bounds(width, height));
    let after = ray(b, direction).take_while(|pos| pos.in_bounds(width, height));
    out.extend(before.chain(after));
}

fn antinodes_for_frequency2(antennas: &[Pos], width: usize, height: usize, out: &mut HashSet<Pos>) {
    for (i, &a) in antennas.iter().enumerate() {
        for &b in &antennas[(i + 1)..] {
            antinodes_for_line2(a, b, width, height, out);
        }
    }
}

fn part2(input: &Input) -> usize {
    let mut antinodes = HashSet::new();
    for antennas in input.antennas.values() {
        antinodes_for_frequency2(antennas, input.width, input.height, &mut antinodes);
    }
    antinodes.len()
}

#[cfg(test)]
mod tests {
    use crate::{coprime_direction, part1, part2, Pos, Vect};

    const SMALL_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        let input = SMALL_INPUT.parse().unwrap();
        assert_eq!(part1(&input), 14);
    }

    #[test]
    fn test_coprime_direction() {
        assert_eq!(
            coprime_direction(Pos { x: 2, y: 6 }, Pos { x: 14, y: -3 }),
            Vect { x: 4, y: -3 }
        )
    }

    #[test]
    fn test_part2() {
        let input = SMALL_INPUT.parse().unwrap();
        assert_eq!(part2(&input), 34);
    }
}
