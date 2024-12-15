fn main() {
    common::run_with_parser(|text| parse_inputs(&text), part1, part2);
}

#[derive(Debug, Clone, Copy)]
struct Button {
    dx: i64,
    dy: i64,
}

#[derive(Debug, Clone, Copy)]
struct Prize {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Input {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

fn parse_inputs(text: &str) -> Vec<Input> {
    text.split("\n\n")
        .map(|text| parse_one_input(text).expect("invalid input"))
        .collect()
}

fn parse_one_input(text: &str) -> Option<Input> {
    let mut lines = text.lines();

    fn parse_coords(
        line: &str,
        prefix: &str,
        x_prefix: &str,
        y_prefix: &str,
    ) -> Option<(i64, i64)> {
        let numbers = line
            .strip_prefix(prefix)?
            .strip_prefix(": ")?
            .strip_prefix(x_prefix)?;

        let (x_word, tail) = numbers.split_once(", ")?;
        let x = x_word.parse().ok()?;
        let y = tail.strip_prefix(y_prefix)?.parse().ok()?;

        Some((x, y))
    }

    fn parse_button(line: &str, prefix: &str) -> Option<Button> {
        let (dx, dy) = parse_coords(line, prefix, "X+", "Y+")?;
        Some(Button { dx, dy })
    }

    let button_a = parse_button(lines.next()?, "Button A")?;
    let button_b = parse_button(lines.next()?, "Button B")?;

    let (prize_x, prize_y) = parse_coords(lines.next()?, "Prize", "X=", "Y=")?;
    let prize = Prize {
        x: prize_x,
        y: prize_y,
    };

    Some(Input {
        button_a,
        button_b,
        prize,
    })
}

fn find_cost(inputs: impl IntoIterator<Item = Input>) -> u64 {
    inputs
        .into_iter()
        .filter_map(|input| {
            let (a, b) = solve_one(input)?;
            Some(a * 3 + b)
        })
        .sum()
}

fn solve_one(input: Input) -> Option<(u64, u64)> {
    // solve a * button_a.(dx, dy) + b * button_b.(dx, dy) = prize.(x, y)

    // the following is derived from solving this equation by rewriting it as a matrix equation
    // and taking the inverse

    let Input {
        button_a: ba,
        button_b: bb,
        prize: p,
    } = input;

    let det = ba.dx * bb.dy - ba.dy * bb.dx;
    if det == 0 {
        return None;
    }

    let a = (bb.dy * p.x - bb.dx * p.y) / det;
    let b = (-ba.dy * p.x + ba.dx * p.y) / det;

    if a * ba.dx + b * bb.dx != p.x || a * ba.dy + b * bb.dy != p.y {
        return None;
    }

    Some((a.try_into().ok()?, b.try_into().ok()?))
}

fn part1(input: &[Input]) -> u64 {
    find_cost(input.iter().copied())
}

fn part2(input: &[Input]) -> u64 {
    find_cost(input.iter().map(|&input| Input {
        prize: Prize {
            x: input.prize.x + 10000000000000,
            y: input.prize.y + 10000000000000,
        },
        ..input
    }))
}

#[cfg(test)]
mod tests {
    use crate::{find_cost, parse_inputs};

    const SMALL_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_find_cost() {
        assert_eq!(find_cost(parse_inputs(SMALL_INPUT).iter().copied()), 480);
    }
}
