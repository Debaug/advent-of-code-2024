use std::fmt::Display;

fn main() {
    common::run_with_parser(
        |text| parse_input(&text),
        |numbers| simulate(numbers, 25),
        |_: &[u64]| "nothing yet",
    );
}

struct Numbers {
    head: Option<Box<Node>>,
}

struct Node {
    value: u64,
    next: Option<Box<Node>>,
}

impl Numbers {
    fn len(&self) -> usize {
        let mut cursor = &self.head;
        let mut len = 0;
        while let Some(ref node) = cursor {
            len += 1;
            cursor = &node.next;
        }
        len
    }
}

impl Display for Numbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cursor = &self.head;
        while let Some(ref node) = cursor {
            write!(f, "{} ", node.value)?;
            cursor = &node.next;
        }
        Ok(())
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        // recursion might be overflow the stack
        let mut cursor = self.next.take();
        while let Some(mut node) = cursor {
            cursor = node.next.take();
        }
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect()
}

fn simulate(input: &[u64], steps: u32) -> usize {
    let mut numbers = Numbers { head: None };
    let mut tail = &mut numbers.head;
    for &num in input {
        tail = &mut tail
            .insert(Box::new(Node {
                value: num,
                next: None,
            }))
            .next;
    }

    for i in 0..steps {
        eprintln!("running step {i}... (len = {})", numbers.len());
        step(&mut numbers);
    }

    numbers.len()
}

fn step(numbers: &mut Numbers) {
    let mut cursor = &mut numbers.head;
    while let Some(ref mut node) = cursor {
        cursor = step_number(node);
    }
}

fn step_number(node: &mut Node) -> &mut Option<Box<Node>> {
    if node.value == 0 {
        node.value = 1;
        return &mut node.next;
    }

    let mut n = 0;
    let mut power_10_n = 1;
    let mut power_10_n_halfs = 1;
    loop {
        if (power_10_n..(power_10_n * 10)).contains(&node.value) {
            break;
        }

        n += 1;
        power_10_n *= 10;
        if n % 2 == 1 {
            power_10_n_halfs *= 10;
        }
    }

    if n % 2 == 1 {
        let tail = node.next.take();
        let tail = &mut node
            .next
            .insert(Box::new(Node {
                value: node.value % power_10_n_halfs,
                next: tail,
            }))
            .next;
        node.value /= power_10_n_halfs;
        tail
    } else {
        node.value *= 2024;
        &mut node.next
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, simulate};

    const SMALL_INPUT: &str = "125 17";

    #[test]
    fn test_simulate() {
        let input = parse_input(SMALL_INPUT);
        assert_eq!(simulate(&input, 25), 55312);
    }
}
