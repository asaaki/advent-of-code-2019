// https://adventofcode.com/2019/day/2

use clap::{App, Arg};
use std::fs;
use std::fs::File;
use std::io::Read;

static PART1_NOUN_VERB: (usize, usize) = (12, 2);
static PART2_ANSWER: usize = 19690720;

fn main() -> std::io::Result<()> {
    let clap_app = App::new("AoC day 2")
        .version("1.0")
        .about("Fix the computer")
        .author("@asaaki")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .takes_value(true),
        );
    let matches = clap_app.get_matches();

    let input_file_path = fs::canonicalize(matches.value_of("INPUT").unwrap())?;
    let file = File::open(&input_file_path)?;

    let default_opcodes = read_data(file);

    let part1_opcodes = adjust_noun_and_verb(default_opcodes.clone(), PART1_NOUN_VERB);
    let answer = run_program(part1_opcodes);

    println!("Answer (part1): {:?}", answer[0]);

    // part 2 - very naive, yes I know
    for noun in 0..100 {
        for verb in 0..100 {
            let p2_opcodes = adjust_noun_and_verb(default_opcodes.clone(), (noun, verb));
            let p2_answer = run_program(p2_opcodes);
            if p2_answer[0] == PART2_ANSWER {
                println!("Answer (part2): {:?}", 100 * noun + verb);
            }
        }
    }

    Ok(())
}

fn read_data(mut file: File) -> Vec<usize> {
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    let opcodes: Vec<usize> = buffer
        .trim()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    opcodes
}

fn adjust_noun_and_verb(mut opcodes: Vec<usize>, (noun, verb): (usize, usize)) -> Vec<usize> {
    opcodes[1] = noun;
    opcodes[2] = verb;
    opcodes
}

fn run_program(opcodes: Vec<usize>) -> Vec<usize> {
    let mut prog = Prog {
        done: false,
        pc: 0,
        opcodes: opcodes,
    };
    prog = prog.execute();
    prog.opcodes
}

#[derive(Debug, Clone)]
struct Prog {
    pub done: bool,
    pub pc: usize,
    pub opcodes: Vec<usize>,
}

impl Prog {
    fn execute(mut self) -> Self {
        while !self.done {
            // usually an "int to opcode" mapping is used, but let's keep it simple for now
            match self.opcodes[self.pc] {
                1 => {
                    let (in1, in2, out) = (
                        self.opcodes[self.pc + 1],
                        self.opcodes[self.pc + 2],
                        self.opcodes[self.pc + 3],
                    );
                    self.opcodes[out] = self.opcodes[in1] + self.opcodes[in2];
                    self.pc = self.pc + 4;
                }
                2 => {
                    let (in1, in2, out) = (
                        self.opcodes[self.pc + 1],
                        self.opcodes[self.pc + 2],
                        self.opcodes[self.pc + 3],
                    );
                    self.opcodes[out] = self.opcodes[in1] * self.opcodes[in2];
                    self.pc = self.pc + 4;
                }
                99 => self.done = true,
                op => panic!("Invalid opcode detected: {} @ pos {}", op, self.pc),
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // part1 - code 1 (1 + 1 = 2)
    #[test]
    fn test_part1_code_1() {
        let input = vec![1, 0, 0, 0, 99];
        let output = vec![2, 0, 0, 0, 99];
        assert_eq!(run_program(input), output);
    }

    // part1 - code 2 (3 * 2 = 6)
    #[test]
    fn test_part1_code_2() {
        let input = vec![2, 3, 0, 3, 99];
        let output = vec![2, 3, 0, 6, 99];
        assert_eq!(run_program(input), output);
    }

    // part1 - code 3 (99 * 99 = 9801)
    #[test]
    fn test_part1_code_3() {
        let input = vec![2, 4, 4, 5, 99, 0];
        let output = vec![2, 4, 4, 5, 99, 9801];
        assert_eq!(run_program(input), output);
    }

    // part1 - code 4
    #[test]
    fn test_part1_code_4() {
        let input = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let output = vec![30, 1, 1, 4, 2, 5, 6, 0, 99];
        assert_eq!(run_program(input), output);
    }
}
