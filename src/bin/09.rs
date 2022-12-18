use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn parse(input: &str) -> impl Iterator<Item = (Direction, u32)> + '_ {
    input.lines().map(|l| {
        let (dir, count) = l.split_once(' ').unwrap();
        let dir = match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!(),
        };
        let count = count.parse().unwrap();
        (dir, count)
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut hx, mut hy, mut tx, mut ty): (i32, i32, i32, i32) = (0, 0, 0, 0);
    let mut positions = HashSet::new();
    positions.insert((tx, ty));
    for (dir, count) in parse(input) {
        for _ in 0..count {
            let prev = (hx, hy);
            let (dx, dy) = match dir {
                Direction::Up => (0, 1),
                Direction::Down => (0, -1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };
            (hx, hy) = (hx + dx, hy + dy);
            if tx.abs_diff(hx) > 1 || ty.abs_diff(hy) > 1 {
                (tx, ty) = prev;
                positions.insert((tx, ty));
            }
        }
    }
    Some(positions.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
