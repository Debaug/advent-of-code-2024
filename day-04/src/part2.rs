use crate::{Input, Letter};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum LetterMS {
    M,
    S,
}

impl TryFrom<Letter> for LetterMS {
    type Error = ();
    fn try_from(letter: Letter) -> Result<Self, Self::Error> {
        match letter {
            Letter::M => Ok(Self::M),
            Letter::S => Ok(Self::S),
            _ => Err(()),
        }
    }
}

fn check_cross(input: &Input, x: isize, y: isize) -> bool {
    check_opposites(input.get(x - 1, y - 1), input.get(x + 1, y + 1))
        && check_opposites(input.get(x - 1, y + 1), input.get(x + 1, y - 1))
}

fn check_opposites(a: Option<Letter>, b: Option<Letter>) -> bool {
    fn convert(a: Option<Letter>, b: Option<Letter>) -> Option<(LetterMS, LetterMS)> {
        Some((a?.try_into().ok()?, b?.try_into().ok()?))
    }
    convert(a, b).is_some_and(|(a, b)| a != b)
}

pub fn part2(input: &Input) -> usize {
    let mut matches = 0;
    for (x, y, l) in input.indexed_letters() {
        if l != Letter::A {
            continue;
        }
        if check_cross(input, x, y) {
            matches += 1;
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use crate::{part2::part2, tests::SMALL_INPUT};

    #[test]
    fn test_part2() {
        let input = SMALL_INPUT.parse().unwrap();
        assert_eq!(part2(&input), 9);
    }
}
