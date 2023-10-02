// Advent of Code 2019, Intcode
// (c) aichingert

use std::collections::HashMap;

pub type N = i64;

pub struct VM {
    ptr: usize,
    base: N,
    input: N,
    mem: HashMap<usize, N>,
}

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Status {
    Normal,
    Input,
    Output(N),
    Exit,
}

impl VM {
    pub fn new(opcodes: &Vec<N>, input: N) -> Self {
        let mut mem = HashMap::new();

        for i in 0..opcodes.len() {
            mem.insert(i, opcodes[i]);
        }

        Self {
            ptr: 0,
            base: 0,
            input,
            mem,
        }
    }

    pub fn _get_position(&mut self, dst: usize) -> N {
        *self.mem.entry(dst).or_insert(0)
    }

    pub fn _set_input(&mut self, input: N) {
        self.input = input;
    }

    fn get_registers(&mut self, modes: &Vec<N>) -> Vec<usize> {
        let mut loc = Vec::<usize>::new();

        for i in 1..modes.len() {
            match modes[i] {
                0 => loc.push(*self.mem.entry(self.ptr + i).or_insert(0) as usize),
                1 => loc.push(self.ptr + i),
                2 => loc.push((self.base + *self.mem.entry(self.ptr + i).or_insert(0)) as usize),
                _ => panic!("unsupported mode"),
            }
        }

        loc
    }

    fn get_op_registers(&mut self, modes: &Vec<N>) -> (usize, usize, usize) {
        let loc = self.get_registers(modes);

        self.ptr += 4;
        (loc[2], loc[0], loc[1])
    }

    fn get_io_register(&mut self, modes: &Vec<N>) -> usize {
        self.ptr += 2;

        match modes[1] {
            0 => *self.mem.entry(self.ptr - 1).or_insert(0) as usize,
            1 => self.ptr - 1,
            2 => (self.base + *self.mem.entry(self.ptr - 1).or_insert(0)) as usize,
            _ => panic!("unsupported mode"),
        }
    }

    fn get_jmp_registers(&mut self, modes: &Vec<N>) -> (usize, N) {
        let loc = self.get_registers(modes);

        self.ptr += 3;
        (*self.mem.entry(loc[1]).or_insert(0) as usize, *self.mem.entry(loc[0]).or_insert(0))
    }

    fn parse_modes(&mut self) -> Vec<N> {
        let mut opcode = *self.mem.entry(self.ptr).or_insert(0);
        let mut modes = vec![0;4];
        let mut idx = 1usize;

        modes[0] += opcode % 100;
        opcode /= 100;

        while opcode > 0 {
            modes[idx] = opcode % 10;
            opcode /= 10;
            idx += 1;
        }

        modes
    }

    pub fn _get_next_opcode(&mut self) -> N {
        *self.mem.entry(self.ptr).or_insert(0) % 100
    }

    pub fn exec(&mut self) -> Status {
        let modes = self.parse_modes();

        match modes[0] {
            1 | 2 | 7 | 8 => {
                let (dst, a, b) = self.get_op_registers(&modes);
                let a = *self.mem.entry(a).or_insert(0);
                let b = *self.mem.entry(b).or_insert(0);

                match modes[0] {
                    1 => self.mem.entry(dst).and_modify(|reg| *reg = a + b).or_insert(a + b),
                    2 => self.mem.entry(dst).and_modify(|reg| *reg = a * b).or_insert(a * b),
                    7 => match a < b {
                        true  => self.mem.entry(dst).and_modify(|reg| *reg = 1).or_insert(1),
                        false => self.mem.entry(dst).and_modify(|reg| *reg = 0).or_insert(0),
                    },
                    8 => match a == b {
                        true  => self.mem.entry(dst).and_modify(|reg| *reg = 1).or_insert(1),
                        false => self.mem.entry(dst).and_modify(|reg| *reg = 0).or_insert(0),
                    },
                    _ => unreachable!(),
                };
            }
            3 | 4 | 9 => {
                let dst = self.get_io_register(&modes);

                match modes[0] {
                    3 => { 
                        let input = self.input;
                        self.mem.entry(dst).and_modify(|reg| *reg = input).or_insert(input);
                        return Status::Input; 
                    }
                    4 => return Status::Output(*self.mem.entry(dst).or_insert(0)),
                    9 => self.base += *self.mem.entry(dst).or_insert(0),
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
