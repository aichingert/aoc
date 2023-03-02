// Advent of Code 2016, assembuny runner
// (c) aichingert

use std::collections::HashMap;

pub enum Instr<'i> {
    Cpy(&'i str, &'i str),
    Inc(&'i str),
    Dec(&'i str),
    Jnz(&'i str, &'i str),
    Tgl(&'i str),
    Out(&'i str),
}

pub struct Runner<'r> {
    pub reg: HashMap<&'r str, i32>,
    code: Vec<Instr<'r>>,
    pub out: Vec<i32>,
}

impl<'r> Runner<'r> {
    pub fn new(string: &'r String) -> Self {
        let mut code = Vec::<Instr<'r>>::new();

        for line in string.lines() {
            let values = line.split(' ').collect::<Vec<&'r str>>();

            match values[0] {
                "cpy" => code.push(Instr::Cpy(values[1], values[2])),
                "inc" => code.push(Instr::Inc(values[1])),
                "dec" => code.push(Instr::Dec(values[1])),
                "jnz" => code.push(Instr::Jnz(values[1], values[2])),
                "tgl" => code.push(Instr::Tgl(values[1])),
                "out" => code.push(Instr::Out(values[1])),
                _ => (),
            }
        }

        Self { reg: HashMap::from([("a",0),("b",0),("c",0),("d",0)]), code, out: Vec::<i32>::new() }
    }

    fn jnz(&self, cur: usize, option: &'r str) -> usize {
        match option.parse::<i32>() {
            Ok(val) => (cur as i32 + val - 1) as usize,
            Err(_) => (cur as i32 + self.reg[&option] - 1) as usize,
        }
    }

    fn tgl(&mut self, cur: usize, offset: i32) {
        if cur as i32 + offset < 0 || (cur as i32 + offset) as usize >= self.code.len() {
            return;
        }
    
        let loc = (cur as i32 + offset) as usize;

        match &self.code[loc] {
            Instr::Cpy(a,b) => self.code[loc] = Instr::Jnz(a,b),
            Instr::Inc(a) => self.code[loc] = Instr::Dec(a),
            Instr::Dec(a) => self.code[loc] = Instr::Inc(a),
            Instr::Jnz(a,b) => self.code[loc] = Instr::Cpy(a, b),
            Instr::Tgl(a) => self.code[loc] = Instr::Inc(a),
            Instr::Out(a) => self.code[loc] = Instr::Inc(a),
        };
    }

    pub fn exec(&mut self, out: &'r str) -> i32 {
        let mut pointer: usize = 0;

        while pointer < self.code.len() {
            match &self.code[pointer] {
                Instr::Cpy(a, b) => if b.parse::<i32 >().is_err() { *self.reg.get_mut(b).unwrap() = match a.parse::<i32>() {
                    Ok(res) => res,
                    Err(_) => self.reg[a],
                }},
                Instr::Inc(a) => *self.reg.get_mut(a).unwrap() += 1,
                Instr::Dec(a) => *self.reg.get_mut(a).unwrap() -= 1,
                Instr::Jnz(a,b) => pointer = match a.parse::<i32>() {
                    Ok(res) => if res != 0 { self.jnz(pointer, b) } else { pointer },
                    Err(_e) => if self.reg[a] != 0 { self.jnz(pointer, b) } else { pointer },
                },
                Instr::Tgl(a) => match a.parse::<i32>() {
                    Ok(val) => { self.tgl(pointer, val); },
                    Err(_e) => { self.tgl(pointer, self.reg[a]); }
                },
                Instr::Out(a) => match a.parse::<i32>() {
                    Ok(val) => self.out.push(val),
                    Err(_e) => self.out.push(self.reg[a]),
                },
            }

            if self.out.len() > 999 { return 0; }

            pointer += 1;
        }

        self.reg[&out]
    }
}
