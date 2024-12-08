use std::cmp::Ordering;

use crate::{data::*, *};

pub fn fix_update(update: &[Page], forward_adjacency: &PageMap<PageSet>) -> Vec<Page> {
    let mut update = update.to_vec();
    update.sort_by(|&a, &b| {
        if a == b {
            Ordering::Equal
        } else if forward_adjacency
            .get(a)
            .is_some_and(|next| next.contains(b))
        {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    update
}

pub fn part2(input: &Input) -> u32 {
    input
        .updates
        .iter()
        .map(|update| (update, fix_update(update, &input.forward_adjacency)))
        .filter(|(update, fixed)| update.as_slice() != fixed)
        .map(|(_, fixed)| fixed[fixed.len() / 2].to_u32())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part2::part2, tests::SMALL_INPUT, Input};

    #[test]
    fn test_part2() {
        let input: Input = SMALL_INPUT.parse().unwrap();
        assert_eq!(part2(&input), 123);
    }
}
