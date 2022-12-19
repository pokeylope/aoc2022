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

#[allow(dead_code)]
fn draw(rope: &[(i32, i32)], positions: &HashSet<(i32, i32)>) {
    let mut map: simple_matrix::Matrix<char> =
        simple_matrix::Matrix::from_iter(21, 26, std::iter::repeat('.'));
    const X_OFFSET: i32 = 11;
    const Y_OFFSET: i32 = 5;
    for (x, y) in positions {
        map.set((y + Y_OFFSET) as usize, (x + X_OFFSET) as usize, '#');
    }
    map.set(Y_OFFSET as usize, X_OFFSET as usize, 's');
    for (i, (x, y)) in rope.iter().enumerate().rev() {
        let c = if i == 0 {
            'H'
        } else {
            char::from_digit(i as u32, 10).unwrap()
        };
        map.set((y + Y_OFFSET) as usize, (x + X_OFFSET) as usize, c);
    }
    println!("{}", positions.len());
    for i in (0..map.rows()).rev() {
        let row = map.get_row(i).unwrap();
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

fn simulate(input: &str, count: usize) -> usize {
    let mut rope: Vec<(i32, i32)> = Vec::from_iter(std::iter::repeat((0, 0)).take(count));
    let mut positions: HashSet<(i32, i32)> = HashSet::new();
    positions.insert(*rope.last().unwrap());
    for (dir, count) in parse(input) {
        println!("{dir:?} {count}");
        for _ in 0..count {
            let (hx, hy) = rope[0];
            let (dx, dy) = match dir {
                Direction::Up => (0, 1),
                Direction::Down => (0, -1),
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
            };
            let (mut px, mut py) = (hx + dx, hy + dy);
            rope[0] = (px, py);
            for (tx, ty) in rope.iter_mut().skip(1) {
                if tx.abs_diff(px) > 1 || ty.abs_diff(py) > 1 {
                    *tx += (px - *tx).signum();
                    *ty += (py - *ty).signum();
                    (px, py) = (*tx, *ty);
                } else {
                    break;
                }
            }
            positions.insert(*rope.last().unwrap());
            //draw(&rope, &positions);
        }
    }
    positions.len()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(simulate(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(simulate(input, 10))
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
        assert_eq!(part_one(&input), Some(/*13*/ 88));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
