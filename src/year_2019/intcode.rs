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
        let opcode = self.read_opcode();
        self.process_opcode(&opcode);
    }

    fn read_opcode(&self) -> Opcode {
        let opcode_id = self.memory[self.pc];
        match opcode_id {
            1 => Opcode::Add(AddParams {
                    src1: self.memory[self.pc + 1],
                    src2: self.memory[self.pc + 2],
                    dst: self.memory[self.pc + 3],
                }),
            2 => Opcode::Mult(MultParams {
                    src1: self.memory[self.pc + 1],
                    src2: self.memory[self.pc + 2],
                    dst: self.memory[self.pc + 3],
                }),
            99 => Opcode::Halt,
            _ => panic!(),
        }
    }

    fn process_opcode(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Add(ref add_params) => {
                self.memory[add_params.dst] = self.memory[add_params.src1] + self.memory[add_params.src2];
                self.pc += 4;
            }
            Opcode::Mult(ref mult_params) => {
                self.memory[mult_params.dst] = self.memory[mult_params.src1] * self.memory[mult_params.src2];
                self.pc += 4;
            }
            Opcode::Halt => {
                self.halt = true;
            }
        }
    }

    pub fn run(&mut self) {
        while !self.halt {
            self.step();
        }
    }
}

enum Opcode {
    Add(AddParams),
    Mult(MultParams),
    Halt,
}

struct MultParams {
    src1: usize,
    src2: usize,
    dst: usize,
}

struct AddParams {
    src1: usize,
    src2: usize,
    dst: usize,
}
