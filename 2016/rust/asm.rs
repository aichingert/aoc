// Advent of Code 2016, assembuny runner
// (c) aichingert

use std::collections::HashMap;

pub enum Instr {
    Cpy(String, String),
    Inc(String),
    Dec(String),
    Jnz(String,String),
    Tgl(String),
    Out(String),
}

pub struct Runner {
    pub reg: HashMap<String, i128>,
    code: Vec<Instr>,
    pub out: Vec<i128>,
}

impl Runner {
    pub fn new(string: &String) -> Self {
        let mut code = Vec::<Instr>::new();

        for line in string.lines() {
            let values = line.split(' ').collect::<Vec<&str>>();

            match values[0] {
                "cpy" => code.push(Instr::Cpy(values[1].to_string(), values[2].to_string())),
                "inc" => code.push(Instr::Inc(values[1].to_string())),
                "dec" => code.push(Instr::Dec(values[1].to_string())),
                "jnz" => code.push(Instr::Jnz(values[1].to_string(), values[2].to_string())),
                "tgl" => code.push(Instr::Tgl(values[1].to_string())),
                "out" => code.push(Instr::Out(values[1].to_string())),
                _ => (),
            }
        }

        Self { reg: HashMap::from([("a".to_string(),0),("b".to_string(),0),("c".to_string(),0),("d".to_string(),0)]), code, out: Vec::<i128>::new() }
    }

    fn jnz(&self, cur: usize, option: &String) -> usize {
        match option.parse::<i128>() {
            Ok(val) => (cur as i128 + val - 1) as usize,
            Err(_) => (cur as i128 + self.reg[option] - 1) as usize,
        }
    }

    fn tgl(&mut self, cur: usize, offset: i128) {
        if cur as i128 + offset < 0 || (cur as i128 + offset) as usize >= self.code.len() {
            return;
        }
    
        let loc = (cur as i128 + offset) as usize;

        match &self.code[loc] {
            Instr::Cpy(a,b) => self.code[loc] = Instr::Jnz(a.clone(),b.clone()),
            Instr::Inc(a) => self.code[loc] = Instr::Dec(a.clone()),
            Instr::Dec(a) => self.code[loc] = Instr::Inc(a.clone()),
            Instr::Jnz(a,b) => self.code[loc] = Instr::Cpy(a.clone(), b.clone()),
            Instr::Tgl(a) => self.code[loc] = Instr::Inc(a.clone()),
            Instr::Out(a) => self.code[loc] = Instr::Inc(a.clone()),
        };
    }

    pub fn exec(&mut self, out: &String) -> i128 {
        let mut pointer: usize = 0;

        while pointer < self.code.len() {
            match &self.code[pointer] {
                Instr::Cpy(a, b) => if b.parse::<i128>().is_err() { *self.reg.get_mut(b).unwrap() = match a.parse::<i128>() {
                    Ok(res) => res,
                    Err(_) => self.reg[a],
                }},
                Instr::Inc(a) => *self.reg.get_mut(a).unwrap() += 1,
                Instr::Dec(a) => *self.reg.get_mut(a).unwrap() -= 1,
                Instr::Jnz(a,b) => pointer = match a.parse::<i128>() {
                    Ok(res) => if res != 0 { self.jnz(pointer, b) } else { pointer },
                    Err(_e) => if self.reg[a] != 0 { self.jnz(pointer, b) } else { pointer },
                },
                Instr::Tgl(a) => match a.parse::<i128>() {
                    Ok(val) => { self.tgl(pointer, val); },
                    Err(_e) => { self.tgl(pointer, self.reg[a]); }
                },
                Instr::Out(a) => match a.parse::<i128>() {
                    Ok(val) => self.out.push(val),
                    Err(_e) => self.out.push(self.reg[a]),
                },
            }

            if self.out.len() > 999 { return 0; }

            pointer += 1;
        }

        self.reg[out]
    }
}
