use itertools::Itertools;
use std::collections::HashSet;

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => 1 + item as u32 - 'a' as u32,
        'A'..='Z' => 27 + item as u32 - 'A' as u32,
        _ => panic!(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        let (a, b) = line.split_at(line.len() / 2);
        let first: HashSet<_> = a.chars().collect();
        let second: HashSet<_> = b.chars().collect();
        let shared = *first.intersection(&second).exactly_one().unwrap();

        total += priority(shared);
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| l.chars().collect::<HashSet<_>>())
            .chunks(3)
            .into_iter()
            .map(|g| {
                priority(
                    *g.reduce(|a, b| a.intersection(&b).copied().collect())
                        .unwrap()
                        .iter()
                        .exactly_one()
                        .unwrap(),
                )
            })
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
