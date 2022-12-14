use itertools::Itertools;

enum Opcode {
    Noop,
    Addx(i32),
}

fn parse(input: &str) -> Vec<Opcode> {
    let mut prog = vec![];
    for line in input.lines() {
        let mut tokens = line.split(' ');
        let mnem = tokens.next().unwrap();
        let insn = match mnem {
            "noop" => Opcode::Noop,
            "addx" => Opcode::Addx(tokens.next().unwrap().parse().unwrap()),
            _ => panic!("Unexpected instruction"),
        };
        prog.push(insn);
    }
    prog
}

struct Vm<'a> {
    prog: &'a [Opcode],
    cycle: usize,
    pc: usize,
    x: i32,
    current: Option<&'a Opcode>,
    delay: usize,
}

impl Vm<'_> {
    fn new(prog: &[Opcode]) -> Vm {
        Vm {
            prog,
            cycle: 0,
            pc: 0,
            x: 1,
            current: None,
            delay: 0,
        }
    }

    fn decode(&mut self) -> bool {
        assert!(self.current.is_none());
        if self.pc >= self.prog.len() {
            return false;
        }
        let insn = &self.prog[self.pc];
        self.current = Some(insn);
        self.delay = match insn {
            Opcode::Noop => 0,
            Opcode::Addx(_) => 1,
        };
        true
    }

    fn commit(&mut self) {
        let insn = self.current.unwrap();
        match insn {
            Opcode::Noop => (),
            Opcode::Addx(val) => {
                self.x += val;
            }
        }
        self.current = None;
        self.pc += 1;
    }

    fn tick(&mut self) -> bool {
        if self.current.is_some() {
            if self.delay > 0 {
                self.delay -= 1;
            }
        } else {
            if !self.decode() {
                return false;
            }
        }
        if self.delay == 0 {
            self.commit();
        }
        self.cycle += 1;
        true
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let prog = parse(input);
    let mut vm = Vm::new(&prog);
    let mut result = 0;
    let cycles = vec![20, 60, 100, 140, 180, 220];
    let mut elapsed = 0;
    for count in cycles {
        while elapsed < count - 1 {
            vm.tick();
            elapsed += 1;
        }
        let strength = (elapsed + 1) * vm.x;
        let pc = vm.pc;
        println!("{elapsed} ({pc}): {elapsed} * {} = {strength}", &vm.x);
        result += strength;
    }
    Some(result)
}

const CRT_WIDTH: usize = 40;

fn display(crt: &[bool]) {
    for line in &crt.iter().chunks(CRT_WIDTH) {
        for pixel in line {
            print!("{}", if *pixel { '#' } else { '.' });
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<bool> {
    let prog = parse(input);
    let mut vm = Vm::new(&prog);
    let mut crt: Vec<bool> = vec![false; 240];
    for i in 0.. {
        if vm.x.abs_diff(i % CRT_WIDTH as i32) <= 1 {
            crt[i as usize] = true;
        }
        if !vm.tick() {
            break;
        }
    }
    println!();
    display(&crt);
    Some(true)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(true));
    }
}
