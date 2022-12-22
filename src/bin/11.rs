use std::collections::VecDeque;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
enum Operation {
    Plus,
    Times,
}

#[derive(Debug)]
enum Operand {
    Number(u32),
    Self_,
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    operand: Operand,
    test: u32,
    if_true: usize,
    if_false: usize,
}

fn parse(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let pat = Regex::new(
        r"^Monkey \d+:
  Starting items: (.+)
  Operation: new = old (.) (.+)
  Test: divisible by (\d+)
    If true: throw to monkey (\d+)
    If false: throw to monkey (\d+)",
    )
    .unwrap();
    for monkey in input.split("\n\n") {
        let m = pat.captures(monkey).unwrap();
        let items = m
            .get(1)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|i| i.parse().unwrap())
            .collect();
        let operation = match m.get(2).unwrap().as_str() {
            "+" => Operation::Plus,
            "*" => Operation::Times,
            _ => panic!(),
        };
        let operand = match m.get(3).unwrap().as_str() {
            "old" => Operand::Self_,
            n => Operand::Number(n.parse().unwrap()),
        };
        let test = m.get(4).unwrap().as_str().parse().unwrap();
        let if_true = m.get(5).unwrap().as_str().parse().unwrap();
        let if_false = m.get(6).unwrap().as_str().parse().unwrap();
        monkeys.push(Monkey {
            items,
            operation,
            operand,
            test,
            if_true,
            if_false,
        });
    }
    monkeys
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys = parse(input);
    let mut counts = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let m = &mut monkeys[i];
            let mut passed = Vec::with_capacity(m.items.len());
            for mut worry in m.items.drain(0..) {
                counts[i] += 1;
                let operand = match m.operand {
                    Operand::Number(n) => n,
                    Operand::Self_ => worry,
                };
                worry = match m.operation {
                    Operation::Plus => worry + operand,
                    Operation::Times => worry * operand,
                };
                worry /= 3;
                let next = if worry % m.test == 0 {
                    m.if_true
                } else {
                    m.if_false
                };
                passed.push((next, worry));
            }
            for (next, worry) in passed {
                monkeys[next].items.push_back(worry);
            }
        }
    }
    Some(counts.iter().sorted().rev().take(2).product())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
