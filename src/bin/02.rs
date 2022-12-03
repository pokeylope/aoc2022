#[derive(Clone, Copy, Debug)]
enum Throw {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy, Debug)]
enum Outcome {
    AWin = 0,
    BWin = 6,
    Draw = 3,
}

use crate::Outcome::*;
use crate::Throw::*;

fn outcome(a: Throw, b: Throw) -> Outcome {
    match a {
        Rock => match b {
            Rock => Draw,
            Paper => BWin,
            Scissors => AWin,
        },
        Paper => match b {
            Rock => AWin,
            Paper => Draw,
            Scissors => BWin,
        },
        Scissors => match b {
            Rock => BWin,
            Paper => AWin,
            Scissors => Draw,
        },
    }
}

fn match_score(a: Throw, b: Throw) -> u32 {
    b as u32 + outcome(a, b) as u32
}

fn outcome_choice(a: Throw, o: Outcome) -> Throw {
    match a {
        Rock => match o {
            AWin => Scissors,
            BWin => Paper,
            Draw => Rock,
        },
        Paper => match o {
            AWin => Rock,
            BWin => Scissors,
            Draw => Paper,
        },
        Scissors => match o {
            AWin => Paper,
            BWin => Rock,
            Draw => Scissors,
        },
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.lines() {
        let (a, b) = line.split_once(' ').unwrap();
        let a = match a {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!(),
        };
        let b = match b {
            "X" => Rock,
            "Y" => Paper,
            "Z" => Scissors,
            _ => panic!(),
        };

        score += match_score(a, b);
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score = 0;
    for line in input.lines() {
        let (a, o) = line.split_once(' ').unwrap();
        let a = match a {
            "A" => Rock,
            "B" => Paper,
            "C" => Scissors,
            _ => panic!(),
        };
        let o = match o {
            "X" => AWin,
            "Y" => Draw,
            "Z" => BWin,
            _ => panic!(),
        };

        let b = outcome_choice(a, o);
        score += match_score(a, b);
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
