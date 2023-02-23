// Advent of Code 2016, assembuny runner
// (c) aichingert

use std::collections::HashMap;

pub enum Instr {
    Cpy(String, String),
    Inc(String),
    Dec(String),
    Jnz(String,String),
}

pub struct Runner {
    pub reg: HashMap<String, i128>,
    code: Vec<Instr>,
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
                _ => (),
            }
        }

        Self { reg: HashMap::from([("a".to_string(),0),("b".to_string(),0),("c".to_string(),0),("d".to_string(),0)]), code }
    }

    pub fn exec(&mut self, out: &String) -> i128 {
        let mut pointer: usize = 0;

        while pointer < self.code.len() {
            match &self.code[pointer] {
                Instr::Cpy(a, b) => *self.reg.get_mut(b).unwrap() = match a.parse::<i128>() {
                    Ok(res) => res,
                    Err(_) => self.reg[a],
                },
                Instr::Inc(a) => *self.reg.get_mut(a).unwrap() += 1,
                Instr::Dec(a) => *self.reg.get_mut(a).unwrap() -= 1,
                Instr::Jnz(a,b) => match a.parse::<i128>() {
                    Ok(res) => if res != 0 {
                        match b.parse::<i128>() {
                            Ok(val) => pointer = (pointer as i128 + val - 1) as usize,
                            Err(_) => pointer = (pointer as i128 + self.reg[b] - 1) as usize,
                        };
                    },
                    Err(_e) => if self.reg[a] != 0 {
                        match b.parse::<i128>() {
                            Ok(val) => pointer = (pointer as i128 + val - 1) as usize,
                            Err(_) => pointer = (pointer as i128 + self.reg[b] - 1) as usize,
                        };
                    },
                },
            }

            pointer += 1;
        }

        self.reg[out]
    }
}
