// Advent of Code 2019, Intcode
// (c) aichingert

type N = i32;

pub struct VM {
    ptr: usize,
    opcodes: Vec<N>,
}

pub enum Status {
    Normal,
    Exit,
}

impl VM {
    pub fn new(opcodes: Vec<N>) -> Self {
        Self {
            ptr: 0,
            opcodes,
        }
    }

    pub fn get_position(&self, dst: usize) -> N {
        self.opcodes[dst]
    }

    fn get_registers(&mut self) -> (usize, usize, usize) {
        self.ptr += 4;
        (self.opcodes[self.ptr - 1] as usize, self.opcodes[self.ptr - 3] as usize, self.opcodes[self.ptr - 2] as usize)
    }

    pub fn execute(&mut self) -> Status {
        if self.ptr >= self.opcodes.len() || self.opcodes[self.ptr] == 99 {
            return Status::Exit;
        }

        match self.opcodes[self.ptr] {
            1 => {
                let (dst, a, b) = self.get_registers();
                self.opcodes[dst] = self.opcodes[a] + self.opcodes[b];
            }
            2 => {
                let (dst, a, b) = self.get_registers();
                self.opcodes[dst] = self.opcodes[a] * self.opcodes[b];
            }
            99 => return Status::Exit,
            _  => panic!("invalid opcode {}", self.opcodes[self.ptr]),
        }

        Status::Normal
    }
}
