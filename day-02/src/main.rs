#[macro_use]
extern crate itertools;

const ADD: usize = 1;
const MULTIPLY: usize = 2;
const HALT: usize = 99;

fn execute(mut program: Vec<usize>) -> usize {
    let mut pc = 0;

    loop {
        match program[pc] {
            HALT => return program[0],
            ADD => {
                let a = program[pc + 1];
                let b = program[pc + 2];
                let c = program[pc + 3];
                program[c] = program[a] + program[b];
                pc += 4;
            }
            MULTIPLY => {
                let a = program[pc + 1];
                let b = program[pc + 2];
                let c = program[pc + 3];
                program[c] = program[a] * program[b];
                pc += 4;
            }
            op => panic!("Unknown opcode {}", op),
        }
    }
}

fn part_1(program: &Vec<usize>) -> usize {
    let mut program = program.clone();
    program[1] = 12;
    program[2] = 2;
    execute(program)
}

fn part_2(program: &Vec<usize>) -> usize {
    for (noun, verb) in iproduct!(0..100, 0..100) {
        let mut program = program.clone();
        program[1] = noun;
        program[2] = verb;
        if execute(program) == 19690720 {
            return 100 * noun + verb;
        }
    }

    panic!("No solution found!")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let program: Vec<usize> = std::fs::read_to_string(filename)
        .unwrap()
        .split(",")
        .map(|s| s.trim())
        .map(|s| usize::from_str_radix(s, 10).unwrap())
        .collect();

    println!("part-1 = {}", part_1(&program));
    println!("part-2 = {}", part_2(&program));
}
