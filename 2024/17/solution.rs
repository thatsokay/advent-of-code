use std::env;
use std::ffi::OsString;
use std::fs;
use std::process;

fn main() {
    match env::args_os().nth(1) {
        Some(file_path) => {
            let input = parse_input(file_path);
            println!("{}", part1(&input));
            println!("{}", part2(&input));
        }
        None => {
            eprintln!("expected 1 argument, but got none");
            process::exit(1);
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    a: u32,
    b: u32,
    c: u32,
    program: Vec<u8>,
    ip: usize,
}

type Input = Computer;

impl Computer {
    fn new(registers: [u32; 3], program: Vec<u8>) -> Self {
        Self {
            a: registers[0],
            b: registers[1],
            c: registers[2],
            program,
            ip: 0,
        }
    }

    fn execute_instruction(&mut self) -> Option<Option<u8>> {
        if self.ip >= self.program.len() {
            return None;
        }
        match self.program[self.ip] {
            0 => {
                self.a /= 2_u32.pow(self.read_combo_operand());
            }
            1 => {
                self.b ^= self.read_literal_operand();
            }
            2 => {
                self.b = self.read_combo_operand() % 8;
            }
            3 => {
                if self.a != 0 {
                    self.ip = self.read_literal_operand() as usize;
                    return Some(None);
                }
            }
            4 => {
                self.b ^= self.c;
            }
            5 => {
                let result = (self.read_combo_operand() % 8) as u8;
                self.ip += 2;
                return Some(Some(result));
            }
            6 => {
                self.b = self.a / 2_u32.pow(self.read_combo_operand());
            }
            7 => {
                self.c = self.a / 2_u32.pow(self.read_combo_operand());
            }
            _ => panic!("Instruction larger than 7"),
        }
        self.ip += 2;
        Some(None)
    }

    fn read_literal_operand(&self) -> u32 {
        self.program[self.ip + 1] as u32
    }

    fn read_combo_operand(&self) -> u32 {
        match self.program[self.ip + 1] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Reserved operand"),
            _ => panic!("Operand larger than 7"),
        }
    }

    fn run(&mut self) -> Vec<u8> {
        std::iter::from_fn(|| self.execute_instruction())
            .take(10000)
            .flat_map(|option| option)
            .collect()
    }
}

fn parse_input(file_path: OsString) -> Input {
    let content = fs::read_to_string(file_path).unwrap();
    let (register_content, program_content) = content.split_once("\n\n").unwrap();
    let mut register_values = register_content
        .lines()
        .map(|line| line.split(": ").nth(1).unwrap().parse().unwrap());
    let program = program_content
        .trim()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    Computer::new(
        [
            register_values.next().unwrap(),
            register_values.next().unwrap(),
            register_values.next().unwrap(),
        ],
        program,
    )
}

fn part1(input: &Input) -> String {
    let mut computer = input.clone();
    computer
        .run()
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn part2(input: &Input) -> u32 {
    (0_u32..)
        .map(|a| {
            println!("{}", a);
            a
        })
        .find(|&a| {
            Computer::new([a, input.b, input.c], input.program.clone()).run() == input.program
        })
        .unwrap()
}
