use std::iter;
use permutohedron::heap_recursive;

type Program = Vec<i32>;

enum Mode {
    Position,
    Value,
}

impl Mode {
    pub fn parse(raw: i32) -> Self {
        match raw {
            0 => Mode::Position,
            1 => Mode::Value,
            x => panic!("Unknown mode {}", x),
        }
    }
}

enum Instruction {
    Add(Mode, Mode),
    Multiply(Mode, Mode),
    Input,
    Output,
    Halt,
    JumpIfTrue(Mode, Mode),
    JumpIfFalse(Mode, Mode),
    LessThan(Mode, Mode),
    Equals(Mode, Mode),
}

impl Instruction {
    pub fn parse(mut raw: i32) -> Self {
        use Instruction::*;

        let code = raw % 100;
        raw /= 100;

        match code {
            1 => {
                let mode0 = Mode::parse(raw % 10); raw /= 10;
                let mode1 = Mode::parse(raw % 10);
                Add(mode0, mode1)
            }
            2 => {
                let mode0 = Mode::parse(raw % 10); raw /= 10;
                let mode1 = Mode::parse(raw % 10);
                Multiply(mode0, mode1)
            }
            3 => Input,
            4 => Output,
            5 => {
                let mode0 = Mode::parse(raw % 10); raw /= 10;
                let mode1 = Mode::parse(raw % 10);
                JumpIfTrue(mode0, mode1)
            }
            6 => {
                let mode0 = Mode::parse(raw % 10); raw /= 10;
                let mode1 = Mode::parse(raw % 10);
                JumpIfFalse(mode0, mode1)
            }
            7 => {
                let mode0 = Mode::parse(raw % 10); raw /= 10;
                let mode1 = Mode::parse(raw % 10);
                LessThan(mode0, mode1)
            }
            8 => {
                let mode0 = Mode::parse(raw % 10); raw /= 10;
                let mode1 = Mode::parse(raw % 10);
                Equals(mode0, mode1)
            }
            99 => Halt,
            x => panic!("Unknown instruction {}", x),
        }
    }
}

fn as_addr(val: i32) -> usize {
    match val {
        x if x < 0 => panic!("Negative Position {}", x),
        x => x as usize,
    }
}

struct Machine {
    pub pc: usize,
    pub program: Program,
    pub halted: bool,
}

impl Machine {
    pub fn new(program: Program) -> Self {
        Machine {
            pc: 0,
            program,
            halted: false,
        }
    }

    fn get(&self, mode: Mode, addr: usize) -> i32 {
        match mode {
            Mode::Position => self.program[as_addr(self.program[addr])],
            Mode::Value => self.program[addr],
        }
    }

    fn execute<'a>(&'a mut self, mut input: impl Iterator<Item = i32> + 'a) -> impl Iterator<Item = i32> + 'a {
        iter::from_fn(move || loop {
            assert!(!self.halted);

            match Instruction::parse(self.program[self.pc]) {
                Instruction::Halt => {
                    self.halted = true;
                    return None;
                }
                Instruction::Input => {
                    let p0 = as_addr(self.program[self.pc + 1]);

                    self.program[p0] = match input.next() {
                        None => return None,
                        Some(val) => val,
                    };

                    self.pc += 2;
                }
                Instruction::Output => {
                    let p0 = as_addr(self.program[self.pc + 1]);
                    self.pc += 2;
                    return Some(self.program[p0]);
                }
                Instruction::Add(mode0, mode1) => {
                    let p0 = self.get(mode0, self.pc + 1);
                    let p1 = self.get(mode1, self.pc + 2);
                    let p2 = as_addr(self.program[self.pc + 3]);

                    self.program[p2] = p0 + p1;
                    self.pc += 4;
                }
                Instruction::Multiply(mode0, mode1) => {
                    let p0 = self.get(mode0, self.pc + 1);
                    let p1 = self.get(mode1, self.pc + 2);
                    let p2 = as_addr(self.program[self.pc + 3]);

                    self.program[p2] = p0 * p1;
                    self.pc += 4;
                }
                Instruction::JumpIfTrue(mode0, mode1) => {
                    let p0 = self.get(mode0, self.pc + 1);
                    let p1 = self.get(mode1, self.pc + 2);

                    match p0 {
                        0 => self.pc += 3,
                        _ => self.pc = as_addr(p1),
                    }
                }
                Instruction::JumpIfFalse(mode0, mode1) => {
                    let p0 = self.get(mode0, self.pc + 1);
                    let p1 = self.get(mode1, self.pc + 2);

                    match p0 {
                        0 => self.pc = as_addr(p1),
                        _ => self.pc += 3,
                    }
                }
                Instruction::LessThan(mode0, mode1) => {
                    let p0 = self.get(mode0, self.pc + 1);
                    let p1 = self.get(mode1, self.pc + 2);
                    let p2 = as_addr(self.program[self.pc + 3]);

                    self.program[p2] = if p0 < p1 { 1 } else { 0 };
                    self.pc += 4;
                }
                Instruction::Equals(mode0, mode1) => {
                    let p0 = self.get(mode0, self.pc + 1);
                    let p1 = self.get(mode1, self.pc + 2);
                    let p2 = as_addr(self.program[self.pc + 3]);

                    self.program[p2] = if p0 == p1 { 1 } else { 0 };
                    self.pc += 4;
                }
            }
        })
    }
}

fn calculate_thrust_simple(p: &Program, phases: &[i32]) -> i32 {
    let mut amp_a = Machine::new(p.clone());
    let mut amp_b = Machine::new(p.clone());
    let mut amp_c = Machine::new(p.clone());
    let mut amp_d = Machine::new(p.clone());
    let mut amp_e = Machine::new(p.clone());

    let data = vec![0];
    let data = amp_a.execute(iter::once(phases[0]).chain(data));
    let data = amp_b.execute(iter::once(phases[1]).chain(data));
    let data = amp_c.execute(iter::once(phases[2]).chain(data));
    let data = amp_d.execute(iter::once(phases[3]).chain(data));
    let mut output = amp_e.execute(iter::once(phases[4]).chain(data));

    output.next().unwrap()
}

fn calculate_thrust_feedback(p: &Program, phases: &[i32]) -> i32 {
    let mut amp_a = Machine::new(p.clone());
    let mut amp_b = Machine::new(p.clone());
    let mut amp_c = Machine::new(p.clone());
    let mut amp_d = Machine::new(p.clone());
    let mut amp_e = Machine::new(p.clone());

    let data = vec![0];
    let data = amp_a.execute(iter::once(phases[0]).chain(data));
    let data = amp_b.execute(iter::once(phases[1]).chain(data));
    let data = amp_c.execute(iter::once(phases[2]).chain(data));
    let data = amp_d.execute(iter::once(phases[3]).chain(data));
    let mut output: Vec<_> = amp_e.execute(iter::once(phases[4]).chain(data)).collect();

    loop {
        let data = output.into_iter();
        let data = amp_a.execute(data);
        let data = amp_b.execute(data);
        let data = amp_c.execute(data);
        let data = amp_d.execute(data);
        output = amp_e.execute(data).collect();
        if amp_e.halted { return *output.last().unwrap(); }
    }
}

fn part_1(program: &Program) -> i32 {
    let mut phases = [0, 1, 2, 3, 4];

    let mut permutations = vec![];
    heap_recursive(&mut phases, |permutation| {
        permutations.push(permutation.to_vec())
    });

    permutations
        .into_iter()
        .map(|phases| calculate_thrust_simple(&program, &phases))
        .max()
        .unwrap()
}

fn part_2(program: &Program) -> i32 {
    let mut phases = [5, 6, 7, 8, 9];

    let mut permutations = vec![];
    heap_recursive(&mut phases, |permutation| {
        permutations.push(permutation.to_vec())
    });

    permutations
        .into_iter()
        .map(|phases| calculate_thrust_feedback(&program, &phases))
        .max()
        .unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let program: Program = std::fs::read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .map(|s| i32::from_str_radix(s, 10).unwrap())
        .collect();

    println!("part-1 = {}", part_1(&program));
    println!("part-2 = {}", part_2(&program));
}
