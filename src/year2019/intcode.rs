pub struct IntCode {

    pub memory: Vec<i64>,
    ip: usize,
    halted: bool,
}

impl IntCode{
    pub(crate) fn new(input: &str) -> Self {
        let memory:Vec<_> = input.split(',').map(|x| x.parse::<i64>().unwrap()).collect();
        IntCode { memory:memory.clone(),ip: 0, halted: false }
    }

    pub(crate) fn start(&mut self){
        while !self.halted {
            self.process();
        }
    }

    fn process(&mut self){
        let pos = self.ip;
        let opcode = self.memory[pos];
        match opcode {
            1 => {
                let a = self.memory[self.memory[pos + 1] as usize];
                let b = self.memory[self.memory[pos + 2] as usize];
                let c = self.memory[pos + 3] as usize;
                self.memory[c] = a + b;
                self.ip += 4;
            }
            2 => {
                let a = self.memory[self.memory[pos + 1] as usize];
                let b = self.memory[self.memory[pos + 2] as usize];
                let c = self.memory[pos + 3] as usize;
                self.memory[c] = a * b;
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
