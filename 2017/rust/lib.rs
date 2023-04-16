// Advent of Code 2017 - lib
// (c) aichingert

use std::collections::HashMap;
pub const SIZE: usize = 256;

pub fn shuffling(list: &mut Vec<u32>, lengths: &Vec<usize>, skip_size: &mut usize, loc: &mut usize) -> u32 {
    for i in 0..lengths.len() {
        let mut col = (0..lengths[i]).map(|idx| list[(*loc + idx) % SIZE]).collect::<Vec<u32>>();
        col = col.iter().rev().map(|n| *n).collect::<Vec<u32>>();

        for j in 0..lengths[i] {
            list[(*loc + j) % SIZE] = col[j];
        }

        *loc = (*loc + lengths[i] + *skip_size) % SIZE;
        *skip_size += 1;
    }

    list[0] * list[1]
}

pub fn knot_hash(hash: &str) -> String {
    let mut lengths = hash.as_bytes().iter().map(|n| *n as usize).collect::<Vec<usize>>();
    lengths.extend([17, 31, 73, 47, 23]);
    let (mut skip_size, mut loc) = (0usize,0usize);
    let mut sparse_hash = (0..SIZE as u32).collect::<Vec<u32>>();
    let mut dense_hash = vec![0;16];
    let mut knot_hash = String::new();

    for _ in 0..64 {
        shuffling(&mut sparse_hash, &lengths, &mut skip_size, &mut loc);
    }

    for i in 0..16 {
        for j in 0..16 {
            dense_hash[i] ^= sparse_hash[i*16+j];
        }
    }

    for i in 0..dense_hash.len() {
        knot_hash.push_str(&format!("{:#04x}", dense_hash[i])[2..]);
    }

    knot_hash
}

pub struct VM {
    pub reg: HashMap<char, i64>,
    asm: Vec<Instr>,
    ip: usize,
    sounds: Vec<i64>,
    rec: Option<i64>,
}

pub enum Instr {
    Snd(Value),
    Rcv(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Jgz(Value, Value),
}

pub enum Value {
    Reg(char),
    Val(i64),
}

impl Value {
    fn parse(s: &str) -> Self {
        match s.parse::<i64>() {
            Ok(val) => Value::Val(val),
            Err(_) => Value::Reg(s.first_char()),
        }
    }
}

impl VM {
    pub fn new(string: &String) -> Self {
        let mut asm = Vec::<Instr>::new();

        for line in string.lines() {
            let vls = line.split(' ').collect::<Vec<&str>>();

            asm.push(match vls[0] {
                "snd" => Instr::Snd(Value::parse(vls[1])),
                "rcv" => Instr::Rcv(Value::parse(vls[1])),
                "set" => Instr::Set(vls[1].first_char(),Value::parse(vls[2])),
                "add" => Instr::Add(vls[1].first_char(),Value::parse(vls[2])),
                "mul" => Instr::Mul(vls[1].first_char(),Value::parse(vls[2])),
                "mod" => Instr::Mod(vls[1].first_char(),Value::parse(vls[2])),
                "jgz" => Instr::Jgz(Value::parse(vls[1]),Value::parse(vls[2])),
                _ => panic!("invalid instr"),
            });
        }

        Self {
            reg: HashMap::new(),
            asm,
            ip: 0,
            sounds: Vec::new(),
            rec: None,
        }
    }

    pub fn step(&mut self) {
        match &self.asm[self.ip] {
            Instr::Snd(s) => match s {
                Value::Val(sound) => self.sounds.push(*sound),
                Value::Reg(reg) => self.sounds.push(*self.reg.entry(*reg).or_insert(0)),
            },
            Instr::Rcv(x) => if match x {
                Value::Val(num) => num,
                Value::Reg(reg) => self.reg.entry(*reg).or_insert(0),
            } != &0 {
                self.rec = Some(self.sounds[self.sounds.len()-1])
            },
            Instr::Set(x,y) => { match y {
                Value::Val(num) => self.reg.insert(*x, *num),
                Value::Reg(reg) => self.reg.insert(*x, self.reg[reg]),
            };},
            _ => (),
        }

        self.ip += 1;
    }
}

trait FirstChar {
    fn first_char(&self) -> char;
}

impl FirstChar for &str {
    fn first_char(&self) -> char {
        self.chars().next().unwrap()
    }
}

fn main() {}
