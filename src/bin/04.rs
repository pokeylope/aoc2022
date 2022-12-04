use itertools::Itertools;

struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn new(start: u32, end: u32) -> Self {
        Assignment { start, end }
    }

    fn parse(s: &str) -> Self {
        let (start, end) = s
            .split('-')
            .map(|s| s.parse::<u32>().unwrap())
            .collect_tuple()
            .unwrap();
        Self::new(start, end)
    }

    fn contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        other.start >= self.start && other.start <= self.end
            || other.end >= self.start && other.end <= self.end
            || other.start <= self.start && other.end >= self.end
    }
}

fn parse(input: &str) -> Vec<(Assignment, Assignment)> {
    input
        .lines()
        .map(|l| l.split(',').map(Assignment::parse).collect_tuple().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse(input)
            .iter()
            .filter(|(a, b)| a.contains(b) || b.contains(a))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse(input).iter().filter(|(a, b)| a.overlaps(b)).count() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
