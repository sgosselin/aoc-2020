use std::env;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

#[derive(Clone, Copy, Debug)]
enum Opcode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn opcode_to_str(op: Opcode) -> String {
    return match op {
        Opcode::Nop(num) => format!("nop {:4}", num),
        Opcode::Acc(num) => format!("acc {:4}", num),
        Opcode::Jmp(num) => format!("jmp {:4}", num),
    };
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

    /// Resets the machine internal state, except for the program.
    pub fn reset(&mut self) {
        self.reg_acc = 0;
        self.reg_pc = 0;

        for i in 0..self.program.len() {
            self.program[i].1 = false;
        }
    }

    /// Runs until an inifinite loop has been detected.
    pub fn run(&mut self, print_log: bool) -> bool {
        self.reset();

        while self.reg_pc < self.program.len() {
            let mut next_acc = self.reg_acc as i32;
            let mut next_pc = self.reg_pc as i32;

            let opcode = &self.program[self.reg_pc];
            if print_log {
                println!("{} |  pc={:4} acc={:4} (executed: {})",
                    opcode_to_str(opcode.0), self.reg_pc, self.reg_acc, opcode.1);
            }

            match opcode {
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
                (op, true) => {
                    return false;
                },
            }

            self.program[self.reg_pc].1 = true;
            self.reg_acc = next_acc;
            self.reg_pc = next_pc as usize;
        }

        return true;
    }

    /// Patches the program to run completely.
    pub fn patch_program(&mut self) -> bool {
        /*
         * The program corruption is either:
         * - A 'jmp' should have been a 'nop'
         * - A 'nop' should have been a 'jmp'
         *
         * The program is small (~600 instructions), with 226 jmp
         * and 61 nop. As such, we can brute force a solution and
         * try to patch all possible instructions.
         *
         * For each instruction that can be patched, we will run the
         * full program and check if it could complete. If not, then
         * we revert the patch and proceed to the next instruction.
         */
        for i in 0..self.program.len() {
            match self.program[i].0 {
                Opcode::Nop(num) => {
                    self.program[i].0 = Opcode::Jmp(num);
                    if self.run(false) {
                        return true;
                    }
                    self.program[i].0 = Opcode::Nop(num);
                },
                Opcode::Jmp(num) => {
                    self.program[i].0 = Opcode::Nop(num);
                    if self.run(false) {
                        return true;
                    }
                    self.program[i].0 = Opcode::Jmp(num);
                },
                _ => {
                },
            }
        }

        return false;
    }

    pub fn get_acc(&self) -> i32 {
        self.reg_acc
    }

    pub fn get_pc(&self) -> usize {
        self.reg_pc
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

    println!("-- part 1 --");
    let success = machine.run(false);
    println!("success: {}", success);
    println!("acc: {}", machine.get_acc());

    println!("-- part 2 --");
    let success = machine.patch_program();
    println!("success: {}", success);
    println!("acc: {}", machine.get_acc());
}
