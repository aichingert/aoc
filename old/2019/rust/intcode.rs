// Advent of Code 2019, Intcode
// (c) aichingert

pub type N = i32;

pub struct VM {
    ptr: usize,
    input: N,
    opcodes: Vec<N>,
}

pub enum Status {
    Normal,
    Output(N),
    Exit,
}

impl VM {
    pub fn new(opcodes: Vec<N>, input: N) -> Self {
        Self {
            ptr: 0,
            input,
            opcodes,
        }
    }

    pub fn get_position(&self, dst: usize) -> N {
        self.opcodes[dst]
    }

    fn get_registers(&mut self, modes: Vec<N>) -> (usize, usize, usize) {
        let mut loc = Vec::<usize>::new();

        for i in 1..modes.len() {
            match modes[i] {
                0 => loc.push(self.opcodes[self.ptr + i] as usize),
                1 => loc.push(self.ptr + i),
                _ => panic!("unsupported mode"),
            }
        }
        self.ptr += 4;

        (self.opcodes[self.ptr - 1] as usize, loc[0], loc[1])
    }

    fn get_io_register(&mut self, modes: Vec<N>) -> usize {
        self.ptr += 2;

        match modes[1] {
            0 => self.opcodes[self.ptr - 1] as usize,
            1 => self.ptr - 1,
            _ => panic!("unsupported mode"),
        }
    }

    fn parse_modes(&self) -> Vec<N> {
        let mut opcode = self.opcodes[self.ptr];
        let mut modes = vec![0;3];
        let mut idx = 1usize;

        modes[0] += opcode % 10;
        opcode /= 10;
        modes[0] += (opcode % 10) * 10;
        opcode /= 10;

        while opcode > 0 {
            modes[idx] = opcode % 10;
            opcode /= 10;
            idx += 1;
        }

        modes
    }

    pub fn execute(&mut self) -> Status {
        if self.ptr >= self.opcodes.len() || self.opcodes[self.ptr] == 99 {
            return Status::Exit;
        }
        let modes = self.parse_modes();

        match modes[0] {
            1 => {
                let (dst, a, b) = self.get_registers(modes);
                self.opcodes[dst] = self.opcodes[a] + self.opcodes[b];
            }
            2 => {
                let (dst, a, b) = self.get_registers(modes);
                self.opcodes[dst] = self.opcodes[a] * self.opcodes[b];
            }
            3 => {
                let dst = self.get_io_register(modes);
                self.opcodes[dst] = self.input;
            }
            4 => {
                let dst = self.get_io_register(modes);
                return Status::Output(self.opcodes[dst]);
            }
            99 => return Status::Exit,
            _  => panic!("invalid opcode {}", self.opcodes[self.ptr]),
        }

        Status::Normal
    }
}
