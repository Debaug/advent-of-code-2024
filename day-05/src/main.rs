use std::{convert::Infallible, str::FromStr};

use data::{Page, PageMap, PageSet};

mod data;
mod part1;
mod part2;

fn main() {
    common::run_with_parser(
        |text| text.parse::<Input>().unwrap(),
        part1::part1,
        part2::part2,
    );
}

struct Input {
    forward_adjacency: PageMap<PageSet>,
    updates: Vec<Vec<Page>>,
}

impl AsRef<Input> for Input {
    fn as_ref(&self) -> &Input {
        self
    }
}

impl FromStr for Input {
    type Err = Infallible;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut lines = text.lines();

        let ordering_rule_section = lines.by_ref().take_while(|line| !line.is_empty());
        let ordering_rules = ordering_rule_section.map(|line| {
            let mut numbers = line.split('|').map(|word| word.parse().ok());
            let before = numbers
                .next()
                .flatten()
                .expect("failed to parse preceding page number in ordering rule");
            let after = numbers
                .next()
                .flatten()
                .expect("failed to parse succeeding page number in ordering rule");
            (before, after)
        });

        let mut forward_adjacency = PageMap::new();
        for (before, after) in ordering_rules {
            forward_adjacency
                .get_or_insert(before, PageSet::new())
                .insert(after);
        }

        let update_section = lines;
        let updates = update_section
            .map(|line| {
                line.split(',')
                    .map(|word| word.parse().expect("failed to parse page number in update"))
                    .collect()
            })
            .collect();

        Ok(Self {
            forward_adjacency,
            updates,
        })
    }
}

#[cfg(test)]
mod tests {

    pub const SMALL_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
}
