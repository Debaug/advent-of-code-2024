use std::iter;

use crate::{Input, Letter};

#[derive(Debug, Clone, Copy)]
struct Direction {
    dx: isize,
    dy: isize,
}

impl Direction {
    fn cardinal_and_diagonal() -> [Self; 8] {
        [
            Self { dx: -1, dy: -1 },
            Self { dx: -1, dy: 0 },
            Self { dx: -1, dy: 1 },
            Self { dx: 0, dy: -1 },
            Self { dx: 0, dy: 1 },
            Self { dx: 1, dy: -1 },
            Self { dx: 1, dy: 0 },
            Self { dx: 1, dy: 1 },
        ]
    }

    fn ray_from(self, mut x: isize, mut y: isize) -> impl Iterator<Item = (isize, isize)> {
        iter::from_fn(move || {
            let result = (x, y);
            x += self.dx;
            y += self.dy;
            Some(result)
        })
    }
}

const XMAS: [Letter; 4] = [Letter::X, Letter::M, Letter::A, Letter::S];

pub fn part1(input: &Input) -> usize {
    let mut matches = 0;

    for (x, y, l) in input.indexed_letters() {
        if l != Letter::X {
            continue;
        }

        for direction in Direction::cardinal_and_diagonal() {
            if direction
                .ray_from(x, y)
                .zip(XMAS)
                .all(|((x, y), letter)| input.get(x, y) == Some(letter))
            {
                matches += 1;
            }
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use crate::{part1, tests::SMALL_INPUT, Input};

    #[test]
    fn test_part1() {
        let input: Input = SMALL_INPUT.parse().unwrap();
        assert_eq!(part1::part1(&input), 18);
    }
}
