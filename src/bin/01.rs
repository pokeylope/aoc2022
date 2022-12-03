use itertools::Itertools;

fn parse(input: &str) -> Vec<u32> {
    let mut elves: Vec<u32> = vec![];
    let mut lines = input.lines();
    loop {
        let mut total = 0;
        for line in lines.by_ref() {
            let Ok(value) = line.parse::<u32>() else { break };
            total += value;
        }
        if total == 0 {
            break;
        }
        elves.push(total);
    }

    elves
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves = parse(input);
    elves.iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves = parse(input);
    Some(elves.iter().sorted().rev().take(3).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
