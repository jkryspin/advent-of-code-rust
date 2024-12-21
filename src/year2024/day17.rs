use std::collections::HashMap;


pub fn part1(input: &str) -> u32
{
    let mut program = Program::from_str(input);
    let result = program.run();

    let result = result.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
    println!("Result: {}", result);
    0
}

pub fn part2(input: &str) -> i128 {
    let mut program = Program::from_str(input);

    let mut todo = vec![(1, 0)];
    let prog = program.instructions.clone().iter().map(|&i|i as i64).collect::<Vec<i64>>();

    let mut lowest = i128::MAX;
    while let Some((i, a)) = todo.pop() {
        for a in a..a+8 {
            program.clear();
            program.registers.insert('A', a);
            let start = prog.len() as i128 - i;
            if start >16 || start <0 {
                continue;
            }

            if program.run() == &prog[start as usize..] {
                todo.push((i + 1, a * 8));
                if i == prog.len() as i128 {
                    println!("{}", a);
                    lowest = lowest.min(a);
                }
            }
        }
    }
    lowest
}

#[derive(Debug)]
struct Program{
    registers: HashMap<char, i128>,
    pointer: usize,
    instructions: Vec<usize>,
    output: Vec<i64>,
}


impl Program {
    fn clear(&mut self){
        self.registers.insert('A', 0);
        self.registers.insert('B', 0);
        self.registers.insert('C', 0);
        self.pointer = 0;
        self.output.clear();
    }
    fn run(&mut self) -> Vec<i64> {
        while self.pointer < self.instructions.len() {
            self.process();
        }
        self.output.clone()
    }
    fn combo_operand(&self, id:usize)->usize{
        if id <= 3 {
            return id;
        }
        if id == 4{
            return self.registers[&'A'] as usize;
        }
        if id == 5{
            return self.registers[&'B'] as usize;
        }
        if id == 6{
            return self.registers[&'C'] as usize;
        }
        unreachable!("Invalid operand id: {}", id);
    }
    fn process(&mut self){
        let opcode= self.instructions[self.pointer];
        let mut increment_pc = true;
        // println!("Processing opcode: {}", opcode);
        // println!("Registers: {:?}", self.registers);
        match opcode {
            0=> {
                self.registers.insert('A', self.registers[&'A'] / 2i128.pow(self.combo_operand(self.instructions[self.pointer + 1]) as u32));
            }
            1=> {
                self.registers.insert('B', self.registers[&'B'] ^ self.instructions[self.pointer + 1] as i128);
            }
            2=> {
                self.registers.insert('B', (self.combo_operand(self.instructions[self.pointer + 1]) % 8) as i64 as i128);
            }
            3=>{
                if self.registers[&'A'] == 0{
                    // do nothing
                }else{
                    self.pointer =self.instructions[self.pointer + 1] as usize;
                    increment_pc = false;
                }

            }
            4=>{
                self.registers.insert('B', self.registers[&'B'] ^ self.registers[&'C']);
            }
            5=>{
                self.output.push((self.combo_operand(self.instructions[self.pointer + 1]) % 8) as i64);
            }
            6=>{
                self.registers.insert('B', self.registers[&'A'] / 2i128.pow(self.combo_operand(self.instructions[self.pointer + 1]) as u32));
            }
            7=>{
                self.registers.insert('C', self.registers[&'A'] / 2i128.pow(self.combo_operand(self.instructions[self.pointer + 1]) as u32));
            }
            _ => {
                unreachable!("Invalid opcode: {}", opcode);
            }
        }


        if increment_pc{
            self.pointer += 2;
        }
    }
    fn from_str(input: &str) -> Self {
        let mut registers = HashMap::new();
        let mut instructions = Vec::new();

        for line in input.lines() {
            if line.starts_with("Register") {
                let parts: Vec<&str> = line.split(':').collect();
                let register = parts[0].chars().last().unwrap();
                let value = parts[1].trim().parse().unwrap();
                registers.insert(register, value);
            } else if line.starts_with("Program:") {
                let parts: Vec<&str> = line.split(':').collect();
                instructions = parts[1].trim().split(',').map(|s| s.parse().unwrap()).collect();
            }
        }

        Program {
            registers,
            pointer: 0,
            instructions,
            output: Vec::new(),
        }
    }
}