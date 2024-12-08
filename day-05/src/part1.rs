use crate::{data::*, *};

fn check_update(update: &[Page], forward_adjacency: &PageMap<PageSet>) -> bool {
    for (&a, &b) in update.iter().zip(update.iter().skip(1)) {
        if !forward_adjacency
            .get(a)
            .is_some_and(|next| next.contains(b))
        {
            return false;
        }
    }

    true
}

pub fn part1(input: &Input) -> u32 {
    input
        .updates
        .iter()
        .filter(|update| check_update(update, &input.forward_adjacency))
        .map(|update| update[update.len() / 2].to_u32())
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{part1::part1, tests::SMALL_INPUT, Input};

    #[test]
    fn test_part1() {
        let input: Input = SMALL_INPUT.parse().unwrap();
        assert_eq!(part1(&input), 143);
    }
}
