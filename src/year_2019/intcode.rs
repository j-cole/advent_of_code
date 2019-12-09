pub struct Intcode {
    rom: Vec<usize>,
    memory: Vec<usize>,
    pc: usize,
    halt: bool,
}

impl Intcode {
    pub fn new(initial: Vec<usize>) -> Result<Self, &'static str> {
        if initial.len() == 0 {
            Err("Initial memory must contain data.")
        } else {
            Ok(Intcode {
                rom: initial,
                memory: vec![],
                pc: 0,
                halt: true,
            })
        }
    }

    pub fn read_output(&self) -> usize {
        self.memory[0]
    }

    pub fn write_noun(&mut self, noun: usize) -> bool {
        let mut write_sucessful = false;
        if 1 <= self.memory.len() {
            self.memory[1] = noun;
            write_sucessful = true;
        }
        write_sucessful
    }

    pub fn write_verb(&mut self, verb: usize) -> bool {
        let mut write_sucessful = false;
        if 2 <= self.memory.len() {
            self.memory[2] = verb;
            write_sucessful = true;
        }
        write_sucessful
    }

    pub fn flash(&mut self, new_rom: Vec<usize>) {
        self.rom = new_rom;
    }

    pub fn dump(&self) -> Vec<usize> {
        self.memory.clone()
    }

    pub fn reset(&mut self) -> &mut Self {
        self.memory = self.rom.clone();
        self.pc = 0;
        self.halt = false;
        self
    }

    pub fn step(&mut self) {
        if self.halt { 
            return;
        }
        if self.memory[self.pc] == 1 {
            // process Add
            let src1 = self.memory[self.pc + 1];
            let src2 = self.memory[self.pc + 2];
            let dest = self.memory[self.pc + 3];
            self.memory[dest] = self.memory[src1] + self.memory[src2];
            self.pc += 4;
        } else if self.memory[self.pc] == 2 {
            // process Multiply
            let src1 = self.memory[self.pc + 1];
            let src2 = self.memory[self.pc + 2];
            let dest = self.memory[self.pc + 3];
            self.memory[dest] = self.memory[src1] * self.memory[src2];
            self.pc += 4;
        } else if self.memory[self.pc] == 99 {
            // process Finish
            self.halt = true;
        } else {
            panic!();
        }
    }

    pub fn run(&mut self) {
        while !self.halt {
            self.step();
        }
    }
}
