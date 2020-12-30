use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Debug)]
enum Opcode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug)]
struct Machine {
    program: Vec<(Opcode, bool)>,
    reg_acc: i32,
    reg_pc: usize,
}

impl Machine {
    pub fn from_file(path: &str) -> Machine {
        let f = File::open(path)
            .expect("could not open the input file");

        let mut program:Vec<(Opcode, bool)> = Vec::new();

        for data in BufReader::new(f).lines() {
            let opcode_str = data.unwrap();

            let num = &opcode_str[4..].parse::<i32>().
                expect("malformed instruction (bad operand)");

            // Decode the opcode.
            match &opcode_str[0..3] {
                "acc" =>
                    program.push( (Opcode::Acc(*num), false) ),
                "jmp" =>
                    program.push( (Opcode::Jmp(*num), false) ),
                "nop" =>
                    program.push( (Opcode::Nop(*num), false) ),
                _ =>
                    panic!("malformed instruction (unknown opcode)"),
            }
        }

        return Machine {
            program: program,
            reg_acc: 0,
            reg_pc: 0,
        };
    }

    /// Runs until an inifinite loop has been detected.
    pub fn run(&mut self) -> bool {
        while self.reg_pc < self.program.len() {
            let mut next_acc = self.reg_acc as i32;
            let mut next_pc = self.reg_pc as i32;

            match self.program[self.reg_pc] {
                (Opcode::Nop(_num), false) => {
                    next_pc += 1;
                },
                (Opcode::Acc(num), false) => {
                    next_acc += num;
                    next_pc += 1;
                },
                (Opcode::Jmp(num), false) => {
                    next_pc += num;
                },
                (_, true) => {
                    return false;
                },
            }

            self.program[self.reg_pc].1 = true;
            self.reg_acc = next_acc;
            self.reg_pc = next_pc as usize;
        }

        return true;
    }

    pub fn get_acc(&self) -> i32 {
        self.reg_acc
    }
}

fn main() {
    let args: Vec<String> = env::args()
        .collect();
    if args.len() != 2 {
        println!("usage: {} [path]", args[0]);
        return;
    }

    let mut machine = Machine::from_file(&args[1]);

    let success = machine.run();
    println!("success: {}", success);
    println!("acc: {}", machine.get_acc());
}
