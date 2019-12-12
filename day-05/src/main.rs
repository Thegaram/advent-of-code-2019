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

        let code = raw % 100; raw /= 100;

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

fn read(input: &mut impl Iterator<Item = i32>) -> i32 {
    match input.next() {
        Some(val) => val,
        None => panic!("No more input!"),
    }
}

fn execute(mut program: Vec<i32>, mut input: impl Iterator<Item = i32>) -> impl Iterator<Item = i32> {
    let mut pc = 0;

    std::iter::from_fn(move || {
        loop {
            match Instruction::parse(program[pc]) {
                Instruction::Halt => return None,
                Instruction::Input => {
                    let addr = as_addr(program[pc + 1]);
                    let value = read(&mut input);
                    program[addr] = value;
                    pc += 2;
                }
                Instruction::Output => {
                    let addr = as_addr(program[pc + 1]);
                    let value = program[addr];
                    pc += 2;
                    return Some(value)
                }
                Instruction::Add(mode0, mode1) => {
                    let addr = as_addr(program[pc + 3]);

                    let p1 = match mode0 {
                        Mode::Position => program[as_addr(program[pc + 1])],
                        Mode::Value => program[pc + 1],
                    };

                    let p2 = match mode1 {
                        Mode::Position => program[as_addr(program[pc + 2])],
                        Mode::Value => program[pc + 2],
                    };

                    program[addr] = p1 + p2;
                    pc += 4;
                }
                Instruction::Multiply(mode0, mode1) => {
                    let addr = as_addr(program[pc + 3]);

                    let p1 = match mode0 {
                        Mode::Position => program[as_addr(program[pc + 1])],
                        Mode::Value => program[pc + 1],
                    };

                    let p2 = match mode1 {
                        Mode::Position => program[as_addr(program[pc + 2])],
                        Mode::Value => program[pc + 2],
                    };

                    program[addr] = p1 * p2;
                    pc += 4;
                }
                Instruction::JumpIfTrue(mode0, mode1) => {
                    let p1 = match mode0 {
                        Mode::Position => program[as_addr(program[pc + 1])],
                        Mode::Value => program[pc + 1],
                    };

                    let p2 = match mode1 {
                        Mode::Position => program[as_addr(program[pc + 2])],
                        Mode::Value => program[pc + 2],
                    };

                    match p1 {
                        0 => pc += 3,
                        _ => pc = as_addr(p2),
                    }
                }
                Instruction::JumpIfFalse(mode0, mode1) => {
                    let p1 = match mode0 {
                        Mode::Position => program[as_addr(program[pc + 1])],
                        Mode::Value => program[pc + 1],
                    };

                    let p2 = match mode1 {
                        Mode::Position => program[as_addr(program[pc + 2])],
                        Mode::Value => program[pc + 2],
                    };

                    match p1 {
                        0 => pc = as_addr(p2),
                        _ => pc += 3,
                    }
                }
                Instruction::LessThan(mode0, mode1) => {
                    let addr = as_addr(program[pc + 3]);

                    let p1 = match mode0 {
                        Mode::Position => program[as_addr(program[pc + 1])],
                        Mode::Value => program[pc + 1],
                    };

                    let p2 = match mode1 {
                        Mode::Position => program[as_addr(program[pc + 2])],
                        Mode::Value => program[pc + 2],
                    };

                    program[addr] = if p1 < p2 { 1 } else { 0 };
                    pc += 4;
                }
                Instruction::Equals(mode0, mode1) => {
                    let addr = as_addr(program[pc + 3]);

                    let p1 = match mode0 {
                        Mode::Position => program[as_addr(program[pc + 1])],
                        Mode::Value => program[pc + 1],
                    };

                    let p2 = match mode1 {
                        Mode::Position => program[as_addr(program[pc + 2])],
                        Mode::Value => program[pc + 2],
                    };

                    program[addr] = if p1 == p2 { 1 } else { 0 };
                    pc += 4;
                }
            }
        }
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let program: Vec<i32> = std::fs::read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .map(|s| i32::from_str_radix(s, 10).unwrap())
        .collect();

    println!("Running diagnostics...");
    for output in execute(program.clone(), std::iter::once(1)) {
        println!("--> {}", output);
    }

    println!("\nRunning diagnostics for thermal radiator controller...");
    for output in execute(program, std::iter::once(5)) {
        println!("--> {}", output);
    }
}
