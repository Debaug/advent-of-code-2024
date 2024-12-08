use std::{collections::HashSet, convert::Infallible, str::FromStr};

fn main() {
    common::run_with_parser(|text| text.parse::<Input>().unwrap(), part1, part2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Direction {
    dx: isize,
    dy: isize,
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VisitMap {
    indices: [Option<usize>; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty(VisitMap),
    Obstacle,
}

#[derive(Debug, Clone)]
struct Board {
    cells: Vec<Cell>,
    width: usize,
    height: usize,
}

#[derive(Debug, Clone)]
struct Input {
    board: Board,
    initial_guard_position: Pos,
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl FromStr for Input {
    type Err = Infallible;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let mut guard_position = None;
        let cells: Vec<Cell> = text
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                assert!(
                    width
                        .replace(line.len())
                        .is_none_or(|width| width == line.len()),
                    "width was not constant"
                );
                line.chars().enumerate().map(move |(x, c)| (x, y, c))
            })
            .map(|(x, y, c)| match c {
                '.' => Cell::Empty(VisitMap::new()),
                '#' => Cell::Obstacle,
                '^' => {
                    assert!(
                        guard_position
                            .replace(Pos {
                                x: x as isize,
                                y: y as isize
                            })
                            .is_none(),
                        "found multiple guards"
                    );
                    Cell::Empty(VisitMap::new())
                }
                c => panic!("invalid character '{c}'"),
            })
            .collect();

        let width = width.unwrap_or(0);
        let height = if width == 0 { 0 } else { cells.len() / width };

        Ok(Self {
            board: Board {
                cells,
                width,
                height,
            },
            initial_guard_position: guard_position.expect("no guard specified"),
        })
    }
}

impl Board {
    fn get_mut(&mut self, pos: Pos) -> Option<&mut Cell> {
        let x: usize = pos.x.try_into().ok().filter(|&x| x < self.width)?;
        let y: usize = pos.y.try_into().ok()?;
        self.cells.get_mut(y * self.height + x)
    }
}

impl Direction {
    const UP: Self = Self {
        dx: 0,
        dy: -1,
        index: 0,
    };

    fn move_pos(self, pos: Pos) -> Pos {
        Pos {
            x: pos.x + self.dx,
            y: pos.y + self.dy,
        }
    }

    fn rotate_quarter_cw(self) -> Direction {
        Self {
            dx: -self.dy,
            dy: self.dx,
            index: (self.index + 1) % 4,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    pos: Pos,
    look_direction: Direction,
    n_visited: usize,
    index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Walked,
    Turned,
    Loop,
    Stop,
}

struct Visit {
    looped: bool,
    n_visited: usize,
}

impl Guard {
    fn new(pos: Pos, look_direction: Direction) -> Self {
        Self {
            pos,
            look_direction,
            n_visited: 0,
            index: 0,
        }
    }

    fn step(&mut self, board: &mut Board) -> Step {
        let in_front_pos = self.look_direction.move_pos(self.pos);
        let Some(in_front_cell) = board.get_mut(in_front_pos) else {
            self.n_visited += 1; // do not mark the last cell on the board as marked
            return Step::Stop;
        };

        match in_front_cell {
            Cell::Obstacle => {
                self.look_direction = self.look_direction.rotate_quarter_cw();
                Step::Turned
            }
            Cell::Empty(_) => {
                let Cell::Empty(visits) = board.get_mut(self.pos).unwrap() else {
                    unreachable!()
                };
                if visits.is_empty() {
                    self.n_visited += 1;
                }
                match visits.entry(self.look_direction) {
                    entry @ None => *entry = Some(self.index),
                    Some(_) => return Step::Loop,
                }
                self.pos = in_front_pos;
                self.index += 1;
                Step::Walked
            }
        }
    }

    fn visit(&mut self, board: &mut Board) -> Visit {
        let looped = loop {
            match self.step(board) {
                Step::Walked | Step::Turned => continue,
                Step::Loop => break true,
                Step::Stop => break false,
            }
        };
        Visit {
            looped,
            n_visited: self.n_visited,
        }
    }
}

impl VisitMap {
    fn new() -> Self {
        Self { indices: [None; 4] }
    }

    fn is_empty(self) -> bool {
        self.indices == [None; 4]
    }

    fn entry(&mut self, direction: Direction) -> &mut Option<usize> {
        &mut self.indices[direction.index]
    }
}

fn part1(input: &Input) -> usize {
    let mut board = input.board.clone();
    Guard::new(input.initial_guard_position, Direction::UP)
        .visit(&mut board)
        .n_visited
}

fn part2(input: &Input) -> usize {
    let initial_guard = Guard::new(input.initial_guard_position, Direction::UP);
    let mut main_board = input.board.clone();
    let mut main_guard = initial_guard;
    let mut obstructions = HashSet::new();
    loop {
        let mut sub_guard = initial_guard;
        let mut sub_board = input.board.clone();
        match main_guard.step(&mut main_board) {
            Step::Loop | Step::Stop => break,
            Step::Turned => {}
            Step::Walked => {
                *sub_board.get_mut(main_guard.pos).unwrap() = Cell::Obstacle;
                if sub_guard.visit(&mut sub_board).looped {
                    obstructions.insert(main_guard.pos);
                }
            }
        }
    }
    obstructions.len()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Input};

    const SMALL_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        let input: Input = SMALL_INPUT.parse().unwrap();
        assert_eq!(part1(&input), 41);
    }

    #[test]
    fn test_part2() {
        let input: Input = SMALL_INPUT.parse().unwrap();
        assert_eq!(part2(&input), 6);
    }
}
