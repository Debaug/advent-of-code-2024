use std::iter;

fn main() {
    common::run_with_parser(|text| parse_input(&text), part1, part2);
}

fn parse_input(input: &str) -> Vec<u8> {
    let input = input.trim();
    assert!(input.chars().all(|c| c.is_ascii_digit()));
    let mut digits = input.as_bytes().to_vec();
    for d in digits.iter_mut() {
        *d -= b'0';
    }
    digits
}

fn part1(input: &[u8]) -> u64 {
    let mut disk: Vec<Option<usize>> = input
        .iter()
        .enumerate()
        .flat_map(|(idx, &n)| {
            if idx % 2 == 0 {
                iter::repeat_n(Some(idx / 2), n.into())
            } else {
                iter::repeat_n(None, n.into())
            }
        })
        .collect();

    let mut i = 0;
    while i < disk.len() {
        if disk.last().unwrap().is_none() {
            disk.pop();
            continue;
        }
        if disk[i].is_none() {
            disk[i] = disk.pop().unwrap();
        }
        i += 1;
    }

    disk.iter()
        .enumerate()
        .map(|(idx, id)| u64::try_from(idx * id.unwrap()).unwrap())
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    start: usize,
    len: usize,
}

impl Segment {
    fn checksum(self, id: usize) -> u64 {
        u64::try_from((self.start..(self.start + self.len)).sum::<usize>() * id).unwrap()
    }
}

fn part2(input: &[u8]) -> u64 {
    let mut start = 0;
    let mut segments: Vec<Segment> = input
        .iter()
        .map(|&len| {
            let len = len.into();
            let result = Segment { start, len };
            start += len;
            result
        })
        .collect();

    let mut checksum = 0;

    for id in (1..=(segments.len() / 2)).rev() {
        let index = id * 2;
        let file = segments[index];

        let Some(matching_space) = segments[1..index]
            .chunks_mut(2)
            .map(|ns| &mut ns[0])
            .find(|segment| segment.len >= file.len)
        else {
            checksum += file.checksum(id);
            continue;
        };

        checksum += Segment {
            start: matching_space.start,
            len: file.len,
        }
        .checksum(id);
        matching_space.start += file.len;
        matching_space.len -= file.len;
    }

    checksum
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, part1, part2};

    const SMALL_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let input = parse_input(SMALL_INPUT);
        assert_eq!(part1(&input), 1928);
    }

    #[test]
    fn test_part2() {
        let input = parse_input(SMALL_INPUT);
        assert_eq!(part2(&input), 2858);
    }
}
