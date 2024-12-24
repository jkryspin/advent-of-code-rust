use crate::year2019::intcode::Mode::{Immediate, Position};

pub struct IntCode {
    pub memory: Vec<i64>,
    ip: usize,
    halted: bool,
    pub(crate) input: i64,
    pub(crate) output: Option<i64>
}

impl IntCode{
    pub(crate) fn new(input: &str) -> Self {
        let memory:Vec<_> = input.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        IntCode { memory:memory.clone(),ip: 0, halted: false, input: 1, output:None }
    }

    pub(crate) fn start(&mut self){
        while !self.halted {
            self.process();
        }
    }
    fn mem(&self, i:usize, op:i64) -> i64 {
        let mode = match i {
            1 => op / 100 % 10,
            2 => op / 1000 % 10,
            3 => op / 10000 % 10,
            _ => panic!("Invalid mode"),
        };
        let m = match mode {
            0 => Position,
            1 => Immediate,
            _ => panic!("Invalid mode: {}", mode),
        };
        let pos = self.ip + i;
        match m {
            Position => self.memory[self.memory[pos] as usize],
            Immediate => self.memory[pos],
        }
    }

    fn process(&mut self){
        let pos = self.ip;
        let op = self.memory[pos];
        let opcode = op % 100;
        println!("Op: {}, Opcode:{}", op, opcode);

        match opcode {
            1 => {
                let a = self.mem(1, op);
                let b = self.mem(2, op);
                let c = self.memory[pos + 3] as usize;
                self.memory[c] = a + b;
                self.ip += 4;
            }
            2 => {
                let a = self.mem(1, op);
                let b = self.mem(2, op);
                let c = self.memory[pos + 3] as usize;
                self.memory[c] = a * b;
                self.ip += 4;
            }
            3 => {
                let a = self.memory[pos + 1] as usize;
                self.memory[a] = self.input;
                self.ip += 2;
            }
            4 => {
                let a = self.mem(1, op);
                self.output = Some(a);
                self.ip += 2;
            }
            5 => {
                let a = self.mem(1, op);
                let b = self.mem(2, op);
                if a != 0 {
                    self.ip = b as usize;
                } else {
                    self.ip += 3;
                }
            }
            6 => {
                let a = self.mem(1, op);
                let b = self.mem(2, op);
                if a == 0 {
                    self.ip = b as usize;
                } else {
                    self.ip += 3;
                }
            }
            7 => {
                let a = self.mem(1, op);
                let b = self.mem(2, op);
                let c = self.memory[pos + 3] as usize;
                self.memory[c] = if a < b { 1 } else { 0 };
                self.ip += 4;
            }
            8 => {
                let a = self.mem(1, op);
                let b = self.mem(2, op);
                let c = self.memory[pos + 3] as usize;
                self.memory[c] = if a == b { 1 } else { 0 };
                self.ip += 4;
            }
            99 => {
                self.halted = true;
            }
            _ => {
                panic!("Invalid opcode: {}, at pos {}", opcode, pos);
            }
        }
    }
}
#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}
