use std::{convert::Infallible, str::FromStr, sync::Arc};

use common::array2::Array2;

fn main() {
    common::run_with_parser(|text| Map::from_str(&text).unwrap(), part1, part2);
}

struct Map {
    heights: Array2<u8>,
}

impl AsRef<Map> for Map {
    fn as_ref(&self) -> &Map {
        self
    }
}

impl FromStr for Map {
    type Err = Infallible;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let heights: Array2<u8> = text
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).expect("invalid height") as u8)
            })
            .collect();

        Ok(Self { heights })
    }
}

#[derive(Debug, Clone, Default)]
struct DestinationSet {
    data: Arc<[(isize, isize)]>,
}

impl DestinationSet {
    fn singleton(x: isize, y: isize) -> Self {
        Self {
            data: Arc::new([(x, y)]),
        }
    }

    fn merge<'a>(sets: impl IntoIterator<Item = &'a DestinationSet>) -> Self {
        let sets: Vec<&DestinationSet> = sets.into_iter().collect();

        if sets.len() == 1 {
            return sets[0].clone(); // share data
        }

        let mut queues: Vec<&[(isize, isize)]> = sets.iter().map(|set| set.data.as_ref()).collect();
        let mut data: Vec<(isize, isize)> = vec![];
        loop {
            // deduplicate
            if let Some(last) = data.last() {
                for queue in &mut queues {
                    while queue.first().is_some_and(|next| next == last) {
                        *queue = &queue[1..];
                    }
                }
            }

            let Some((next, next_queue)) = queues
                .iter_mut()
                .filter_map(|queue| Some((*queue.first()?, queue)))
                .min_by_key(|(next, _)| *next)
            else {
                break;
            };

            data.push(next);
            *next_queue = &next_queue[1..];
        }

        Self { data: data.into() }
    }
}

fn part1(map: &Map) -> usize {
    let mut height_positions: [Vec<(isize, isize)>; 10] = Default::default();
    for (x, y, &height) in map.heights.indexed_iter() {
        height_positions[usize::from(height)].push((x, y));
    }

    let mut destinations = Array2::from_default(map.heights.width(), map.heights.height());
    for &(x, y) in &height_positions[9] {
        *destinations.get_mut(x, y).unwrap() = DestinationSet::singleton(x, y);
    }

    for height in (1..=8).rev() {
        for &(x, y) in &height_positions[height] {
            let neighbor_destinations = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter(|&(u, v)| {
                    map.heights
                        .get(u, v)
                        .is_some_and(|&h| usize::from(h) == height + 1)
                })
                .map(|(u, v)| destinations.get(u, v).unwrap());
            let destinations_from_here = DestinationSet::merge(neighbor_destinations);
            *destinations.get_mut(x, y).unwrap() = destinations_from_here;
        }
    }

    height_positions[0]
        .iter()
        .map(|&(x, y)| {
            let neighbor_destinations = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter(|&(u, v)| map.heights.get(u, v).is_some_and(|&h| usize::from(h) == 1))
                .map(|(u, v)| destinations.get(u, v).unwrap());
            DestinationSet::merge(neighbor_destinations).data.len()
        })
        .sum()
}

fn part2(map: &Map) -> u64 {
    let mut height_positions: [Vec<(isize, isize)>; 10] = Default::default();
    for (x, y, &height) in map.heights.indexed_iter() {
        height_positions[usize::from(height)].push((x, y));
    }

    let mut scores = Array2::from_element(0, map.heights.width(), map.heights.height());
    for &(x, y) in &height_positions[0] {
        *scores.get_mut(x, y).unwrap() = 1;
    }

    for (height, positions) in height_positions.iter().enumerate().skip(1) {
        for &(x, y) in positions {
            *scores.get_mut(x, y).unwrap() = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                .into_iter()
                .filter(|&(u, v)| {
                    map.heights
                        .get(u, v)
                        .is_some_and(|&h| usize::from(h) == height - 1)
                })
                .map(|(u, v)| *scores.get(u, v).unwrap())
                .sum();
        }
    }

    height_positions[9]
        .iter()
        .map(|&(x, y)| scores.get(x, y).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Map};

    const SMALL_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        let map: Map = SMALL_INPUT.parse().unwrap();
        assert_eq!(part1(&map), 36);
    }

    #[test]
    fn test_part2() {
        let map: Map = SMALL_INPUT.parse().unwrap();
        assert_eq!(part2(&map), 81);
    }
}
