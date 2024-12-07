fn main() {
    common::run(part1, part2);
}

fn part1(mut input: &str) -> u32 {
    let mut sum = 0;
    let mut parser = MulParser::new();
    while !input.is_empty() {
        let ParserStep { finished, tail } = parser.step(input);
        if let Some((lhs, rhs)) = finished {
            sum += lhs * rhs;
        }
        input = tail;
    }
    sum
}

fn part2(mut input: &str) -> u32 {
    let mut sum = 0;
    let mut parser = Parser::new();
    while !input.is_empty() {
        let ParserStep { finished, tail } = parser.step(input);
        if let Some((lhs, rhs)) = finished {
            sum += lhs * rhs;
        }
        input = tail;
    }
    sum
}

struct Parser {
    active: Option<MulParser>,
}

#[derive(Debug, Clone, Copy)]
enum MulParser {
    MulLeftParen,
    Lhs,
    Comma { lhs: u32 },
    Rhs { lhs: u32 },
    RightParen { lhs: u32, rhs: u32 },
}

struct ParserStep<'a> {
    finished: Option<(u32, u32)>,
    tail: &'a str,
}

impl Parser {
    fn new() -> Self {
        Self {
            active: Some(MulParser::new()),
        }
    }
}

impl Parser {
    fn step<'a>(&mut self, input: &'a str) -> ParserStep<'a> {
        if let Some(tail) = input.strip_prefix("do()") {
            self.active = Some(MulParser::MulLeftParen);
            return ParserStep {
                finished: None,
                tail,
            };
        }
        if let Some(tail) = input.strip_prefix("don't()") {
            self.active = None;
            return ParserStep {
                finished: None,
                tail,
            };
        }

        if let Some(parser) = &mut self.active {
            return parser.step(input);
        }

        ParserStep {
            finished: None,
            tail: skip_first_char(input),
        }
    }
}

impl MulParser {
    fn new() -> Self {
        Self::MulLeftParen
    }

    fn step<'a>(&mut self, input: &'a str) -> ParserStep<'a> {
        match *self {
            Self::MulLeftParen => ParserStep {
                finished: None,
                tail: self.step_exact(input, "mul(", Self::Lhs).1,
            },

            Self::Lhs => {
                let (lhs, tail) = Self::step_number(input);
                *self = match lhs {
                    Some(lhs) => Self::Comma { lhs },
                    None => Self::MulLeftParen,
                };
                ParserStep {
                    finished: None,
                    tail,
                }
            }

            Self::Comma { lhs } => ParserStep {
                finished: None,
                tail: self.step_exact(input, ",", Self::Rhs { lhs }).1,
            },

            Self::Rhs { lhs } => {
                let (rhs, tail) = Self::step_number(input);
                *self = match rhs {
                    Some(rhs) => Self::RightParen { lhs, rhs },
                    None => Self::MulLeftParen,
                };
                ParserStep {
                    finished: None,
                    tail,
                }
            }

            Self::RightParen { lhs, rhs } => {
                let (success, tail) = self.step_exact(input, ")", Self::MulLeftParen);
                ParserStep {
                    finished: success.then_some((lhs, rhs)),
                    tail,
                }
            }
        }
    }

    fn step_exact<'a>(
        &mut self,
        input: &'a str,
        prefix: &str,
        next_state: MulParser,
    ) -> (bool, &'a str) {
        match input.strip_prefix(prefix) {
            Some(tail) => {
                *self = next_state;
                (true, tail)
            }
            None => {
                *self = Self::MulLeftParen;
                (false, skip_first_char(input))
            }
        }
    }

    fn step_number(input: &str) -> (Option<u32>, &str) {
        let mut number: Option<u32> = None;
        let mut chars = input.chars();
        for _ in 0..3 {
            let Some(digit) = chars.clone().next().and_then(|c| c.to_digit(10)) else {
                break;
            };
            chars.next();
            number = number.map(|number| number * 10 + digit).or(Some(digit));
        }
        if number.is_none() {
            chars.next();
        }
        (number, chars.as_str())
    }
}

fn skip_first_char(input: &str) -> &str {
    let mut chars = input.chars();
    chars.next();
    chars.as_str()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        const SMALL_INPUT: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(SMALL_INPUT), 161);
    }

    #[test]
    fn test_part2() {
        const SMALL_INPUT: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(SMALL_INPUT), 48);
    }
}
