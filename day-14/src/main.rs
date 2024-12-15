use std::{str::FromStr, thread, time::Duration};

use common::{
    array2::Array2,
    math::{Pos, Vect},
};

fn main() {
    common::run_with_parser(|text| parse_input(&text), part1, part2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Robot {
    pos: Pos,
    velocity: Vect,
}

impl FromStr for Robot {
    type Err = ();
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        fn inner(line: &str) -> Option<Robot> {
            let (px, tail) = line.strip_prefix("p=")?.split_once(",")?;
            let (py, tail) = tail.split_once(" v=")?;
            let (vx, tail) = tail.split_once(",")?;
            let vy = tail;
            Some(Robot {
                pos: Pos {
                    x: px.parse().ok()?,
                    y: py.parse().ok()?,
                },
                velocity: Vect {
                    x: vx.parse().ok()?,
                    y: vy.parse().ok()?,
                },
            })
        }
        inner(line).ok_or(())
    }
}

fn parse_input(text: &str) -> Vec<Robot> {
    text.lines()
        .map(|line| line.parse().expect("invalid input"))
        .collect()
}

fn simulate(robot: Robot, steps: isize, width: isize, height: isize) -> Pos {
    (robot.pos + robot.velocity * steps).rem_euclid(width, height)
}

fn part1(robots: &[Robot]) -> usize {
    let end_positions = robots.iter().map(|&robot| simulate(robot, 100, 101, 103));

    let mut first_quadrant = 0;
    let mut second_quadrant = 0;
    let mut third_quadrant = 0;
    let mut fourth_quadrant = 0;
    for Pos { x, y } in end_positions {
        if x < 50 && y < 51 {
            first_quadrant += 1;
        } else if x > 50 && y < 51 {
            second_quadrant += 1;
        } else if x < 50 && y > 51 {
            third_quadrant += 1;
        } else if x > 50 && y > 51 {
            fourth_quadrant += 1;
        }
    }
    first_quadrant * second_quadrant * third_quadrant * fourth_quadrant
}

// fn part2(robots: &[Robot]) -> String {
//     let end_positions = robots
//         .iter()
//         .map(|&robot| simulate(robot, 223020000, 101, 103));

//     let mut canvas = vec![vec![b'.'; 101]; 103];
//     for Pos { x, y } in end_positions {
//         canvas[usize::try_from(y).expect("robot position was out-of-bounds")]
//             [usize::try_from(x).expect("robot position was out-of-bounds")] = b'#';
//     }

//     canvas
//         .into_iter()
//         .map(|line| String::from_utf8(line).unwrap() + "\n")
//         .collect()
// }

fn part2(robots: &[Robot]) -> isize {
    let end = (1..10000)
        .max_by_key(|&second| {
            let end_positions = robots
                .iter()
                .map(|&robot| simulate(robot, second, 101, 103));
            score(end_positions)
        })
        .unwrap();

    let end_positions = robots.iter().map(|&robot| simulate(robot, end, 101, 103));

    let mut canvas = vec![vec![b'.'; 101]; 103];
    for Pos { x, y } in end_positions {
        canvas[usize::try_from(y).expect("robot position was out-of-bounds")]
            [usize::try_from(x).expect("robot position was out-of-bounds")] = b'#';
    }

    let pic: String = canvas
        .into_iter()
        .map(|line| String::from_utf8(line).unwrap() + "\n")
        .collect();

    println!("{pic}");
    end
}

fn score(robots: impl IntoIterator<Item = Pos>) -> usize {
    let weights = Array2::from_fn(101, 103, |x, y| {
        let len = y / 2;
        (x - 50).abs() <= len
    });
    robots
        .into_iter()
        .filter(|pos| *weights.get(pos.x, pos.y).unwrap())
        .count()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use common::math::{Pos, Vect};

    use crate::{simulate, Robot};

    #[test]
    fn test_simulate() {
        let start = Robot {
            pos: Pos::new(2, 4),
            velocity: Vect::new(2, -3),
        };
        let end = simulate(start, 5, 11, 7);
        assert_eq!(end, Pos::new(1, 3));
    }

    #[test]
    fn robot_from_str() {
        assert_eq!(
            Robot::from_str("p=0,4 v=3,-3").expect("failed to parse robot"),
            Robot {
                pos: Pos::new(0, 4),
                velocity: Vect::new(3, -3)
            }
        );
    }
}
