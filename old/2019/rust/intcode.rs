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

    pub fn _get_position(&self, dst: usize) -> N {
        self.opcodes[dst]
    }

    fn get_registers(&mut self, modes: &Vec<N>) -> Vec<usize> {
        let mut loc = Vec::<usize>::new();

        for i in 1..modes.len() {
            match modes[i] {
                0 => loc.push(self.opcodes[self.ptr + i] as usize),
                1 => loc.push(self.ptr + i),
                _ => panic!("unsupported mode"),
            }
        }

        loc
    }

    fn get_op_registers(&mut self, modes: &Vec<N>) -> (usize, usize, usize) {
        let loc = self.get_registers(modes);

        self.ptr += 4;
        (self.opcodes[self.ptr - 1] as usize, loc[0], loc[1])
    }

    fn get_io_register(&mut self, modes: &Vec<N>) -> usize {
        self.ptr += 2;

        match modes[1] {
            0 => self.opcodes[self.ptr - 1] as usize,
            1 => self.ptr - 1,
            _ => panic!("unsupported mode"),
        }
    }

    fn get_jmp_registers(&mut self, modes: &Vec<N>) -> (usize, N) {
        let loc = self.get_registers(modes);

        self.ptr += 3;
        (self.opcodes[loc[1]] as usize, self.opcodes[loc[0]])
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
            1 | 2 | 7 | 8 => {
                let (dst, a, b) = self.get_op_registers(&modes);

                match modes[0] {
                    1 => self.opcodes[dst] = self.opcodes[a] + self.opcodes[b],
                    2 => self.opcodes[dst] = self.opcodes[a] * self.opcodes[b],
                    7 => match self.opcodes[a] < self.opcodes[b] {
                        true  => self.opcodes[dst] = 1,
                        false => self.opcodes[dst] = 0,
                    },
                    8 => match self.opcodes[a] == self.opcodes[b] {
                        true  => self.opcodes[dst] = 1,
                        false => self.opcodes[dst] = 0,
                    },
                    _ => unreachable!(),
                };
            }
            3 | 4 => {
                let dst = self.get_io_register(&modes);

                match modes[0] {
                    3 => self.opcodes[dst] = self.input,
                    4 => return Status::Output(self.opcodes[dst]),
                    _ => unreachable!(),
                };
            }
            5 | 6 => {
                let (dst, value) = self.get_jmp_registers(&modes);

                match modes[0] {
                    5 => if value != 0 { self.ptr = dst; },
                    6 => if value == 0 { self.ptr = dst; },
                    _ => unreachable!(),
                };
            }
            99 => return Status::Exit,
            _ => panic!("invalid op code!"),
        }

        Status::Normal
    }
}
