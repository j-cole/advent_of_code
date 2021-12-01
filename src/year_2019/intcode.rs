pub struct Intcode {
    rom: Vec<i64>,
    memory: Vec<i64>,
    pc: usize,
    halt: bool,
}

impl Intcode {
    pub fn new(initial: Vec<i64>) -> Result<Self, &'static str> {
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

    pub fn read_output(&self) -> i64 {
        self.memory[0]
    }

    pub fn write_noun(&mut self, noun: i64) -> bool {
        let mut write_sucessful = false;
        if 1 <= self.memory.len() {
            self.memory[1] = noun;
            write_sucessful = true;
        }
        write_sucessful
    }

    pub fn write_verb(&mut self, verb: i64) -> bool {
        let mut write_sucessful = false;
        if 2 <= self.memory.len() {
            self.memory[2] = verb;
            write_sucessful = true;
        }
        write_sucessful
    }

    pub fn flash(&mut self, new_rom: Vec<i64>) {
        self.rom = new_rom;
    }

    pub fn dump(&self) -> Vec<i64> {
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
        match opcode_id % 100 {
            1 => Opcode::Add(AddParams {
                src1: if (opcode_id / 100) % 10 == 1 {
                    self.pc + 1
                } else {
                    self.memory[self.pc + 1] as usize
                },
                src2: if (opcode_id / 1000) % 10 == 1 {
                    self.pc + 2
                } else {
                    self.memory[self.pc + 2] as usize
                },
                dst: self.memory[self.pc + 3] as usize,
            }),
            2 => Opcode::Mult(MultParams {
                src1: if (opcode_id / 100) % 10 == 1 {
                    self.pc + 1
                } else {
                    self.memory[self.pc + 1] as usize
                },
                src2: if (opcode_id / 1000) % 10 == 1 {
                    self.pc + 2
                } else {
                    self.memory[self.pc + 2] as usize
                },
                dst: self.memory[self.pc + 3] as usize,
            }),
            3 => Opcode::Input(InputParams {
                dst: self.memory[self.pc + 1] as usize,
            }),
            4 => Opcode::Output(OutputParams {
                src: if (opcode_id / 100) % 10 == 1 {
                    self.pc + 1
                } else {
                    self.memory[self.pc + 1] as usize
                },
            }),
            99 => Opcode::Halt,
            _ => {
                for (index, data) in self.memory.iter().enumerate() {
                    println!("{} - {}", index, data);
                }
                panic!("Unknown opcode: {} at memory: {}", opcode_id, self.pc)
            }
        }
    }

    fn process_opcode(&mut self, opcode: &Opcode) {
        match opcode {
            Opcode::Add(ref add_params) => {
                self.memory[add_params.dst] =
                    self.memory[add_params.src1] + self.memory[add_params.src2];
                self.pc += 4;
            }
            Opcode::Mult(ref mult_params) => {
                self.memory[mult_params.dst] =
                    self.memory[mult_params.src1] * self.memory[mult_params.src2];
                self.pc += 4;
            }
            Opcode::Halt => {
                self.halt = true;
            }
            Opcode::Input(ref input_params) => {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let parsed_input = input.trim().parse::<i64>().unwrap();
                self.memory[input_params.dst] = parsed_input;
                self.pc += 2;
            }
            Opcode::Output(ref output_params) => {
                println!("Output: {}", self.memory[output_params.src]);
                self.pc += 2;
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
    Input(InputParams),
    Output(OutputParams),
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

struct InputParams {
    dst: usize,
}

struct OutputParams {
    src: usize,
}
