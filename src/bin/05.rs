use itertools::Itertools;
use regex::Regex;

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let (diagram, instructions) = input.split_once("\n\n").unwrap();

    let mut diagram = diagram.lines().rev();
    let nums = diagram.next().unwrap();
    let count = (nums.len() + 1) / 4;

    let mut stacks = vec![vec![]; count];

    for line in diagram {
        for (i, stack) in stacks.iter_mut().enumerate() {
            let c = line.as_bytes()[1 + 4 * i] as char;
            if c != ' ' {
                stack.push(c)
            }
        }
    }

    let pat = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let instructions = instructions
        .lines()
        .map(|line| {
            let caps = pat.captures(line).unwrap();
            let count: usize = caps.get(1).unwrap().as_str().parse().unwrap();
            let src: usize = caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let dst: usize = caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1;
            (count, src, dst)
        })
        .collect();

    (stacks, instructions)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut stacks, instructions) = parse(input);
    for (count, src, dst) in instructions {
        for _ in 0..count {
            let b = stacks[src].pop().unwrap();
            stacks[dst].push(b);
        }
    }
    Some(stacks.iter().map(|v| v.last().unwrap()).join(""))
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut stacks, instructions) = parse(input);
    for (count, src, dst) in instructions {
        let i = stacks[src].len() - count;
        let crates = stacks[src].drain(i..).collect_vec();
        stacks[dst].extend(crates);
    }
    Some(stacks.iter().map(|v| v.last().unwrap()).join(""))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}
