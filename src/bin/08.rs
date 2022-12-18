use std::iter::repeat;

use itertools::Itertools;
use simple_matrix::Matrix;

const MAX_HEIGHT: i8 = 9;

type Forest = Matrix<i8>;

fn parse(input: &str) -> Forest {
    let mut lines = input.lines().peekable();
    let size = lines.peek().unwrap().len();
    let mut forest = Matrix::zero(size, size);
    for (row, line) in lines.enumerate() {
        for (col, c) in line.chars().enumerate() {
            forest.set(row, col, c.to_digit(10).unwrap() as i8);
        }
    }
    forest
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn measure_visibility(
    forest: &Forest,
    visibile: &mut Matrix<bool>,
    index: usize,
    direction: Direction,
) {
    let (mut var, delta, stop): (usize, isize, usize) = match direction {
        Direction::Up => (forest.rows() - 1, -1, 0),
        Direction::Down => (0, 1, forest.rows() - 1),
        Direction::Left => (forest.cols() - 1, -1, 0),
        Direction::Right => (0, 1, forest.cols() - 1),
    };
    let mut max = -1;
    while var != stop {
        let (row, col) = match direction {
            Direction::Up | Direction::Down => (var, index),
            Direction::Left | Direction::Right => (index, var),
        };
        let height = forest.get(row, col).unwrap();
        if height > max {
            visibile.set(row, col, true);
            max = height;
        }
        if height == MAX_HEIGHT {
            break;
        }
        var = (var as isize + delta) as usize;
    }
}

fn view_count(forest: &Forest, row: usize, col: usize, direction: Direction) -> usize {
    let height = forest.get(row, col).unwrap();
    let (fixed, mut var, delta, stop): (usize, usize, isize, usize) = match direction {
        Direction::Up => (col, row, -1, 0),
        Direction::Down => (col, row, 1, forest.rows() - 1),
        Direction::Left => (row, col, -1, 0),
        Direction::Right => (row, col, 1, forest.cols() - 1),
    };
    let mut count = 0;
    while var != stop {
        count += 1;
        var = (var as isize + delta) as usize;
        let (row, col) = match direction {
            Direction::Up | Direction::Down => (var, fixed),
            Direction::Left | Direction::Right => (fixed, var),
        };
        let tree = forest.get(row, col).unwrap();
        if tree >= height {
            break;
        }
    }
    count
}

fn measure_score(forest: &Forest, row: usize, col: usize) -> usize {
    view_count(forest, row, col, Direction::Up)
        * view_count(forest, row, col, Direction::Down)
        * view_count(forest, row, col, Direction::Left)
        * view_count(forest, row, col, Direction::Right)
}

fn output(visible: &Matrix<bool>) {
    for i in 0..visible.rows() {
        let row = visible.get_row(i).unwrap();
        for tree in row {
            let c = if *tree { 'x' } else { '.' };
            print!("{}", c);
        }
        println!("");
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let forest = parse(input);
    let mut visible: Matrix<bool> = Matrix::from_iter(forest.rows(), forest.cols(), repeat(false));
    for row in 0..forest.rows() {
        measure_visibility(&forest, &mut visible, row, Direction::Left);
        measure_visibility(&forest, &mut visible, row, Direction::Right);
    }
    for col in 0..forest.cols() {
        measure_visibility(&forest, &mut visible, col, Direction::Up);
        measure_visibility(&forest, &mut visible, col, Direction::Down);
    }
    Some(visible.iter().filter(|&e| *e).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let forest = parse(input);
    (0..forest.rows())
        .cartesian_product(0..forest.cols())
        .map(|(row, col)| measure_score(&forest, row, col))
        .max()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
