// Advent of Code 2017, day 18
// (c) aichingert

use std::collections::VecDeque;

#[derive(Clone)]
pub struct VM {
    pub reg: Vec<i64>,
    pub asm: Vec<Instr>,
    pub ip: usize,
    sounds: VecDeque<i64>,
    frq: Option<i64>,
    rec_reg: Option<char>,
    sended: i64,
}

#[derive(Clone)]
pub enum Instr {
    Snd(Value),
    Rcv(char),
    Set(char, Value),
    Add(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Jnz(Value, Value),
    Jgz(Value, Value),
}

#[derive(Clone)]
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

    fn value(&self, reg: &[i64]) -> i64 {
        match self {
            Self::Val(v) => *v,
            Self::Reg(c) => reg[(*c as u8 - b'a') as usize],
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
                "rcv" => Instr::Rcv(vls[1].first_char()),
                "set" => Instr::Set(vls[1].first_char(),Value::parse(vls[2])),
                "add" => Instr::Add(vls[1].first_char(),Value::parse(vls[2])),
                "sub" => Instr::Sub(vls[1].first_char(),Value::parse(vls[2])),
                "mul" => Instr::Mul(vls[1].first_char(),Value::parse(vls[2])),
                "mod" => Instr::Mod(vls[1].first_char(),Value::parse(vls[2])),
                "jnz" => Instr::Jnz(Value::parse(vls[1]),Value::parse(vls[2])),
                "jgz" => Instr::Jgz(Value::parse(vls[1]),Value::parse(vls[2])),
                _ => panic!("invalid instr"),
            });
        }

        Self {
            reg: vec![0;26],
            asm,
            ip: 0,
            sounds: VecDeque::new(),
            frq: None,
            rec_reg: None,
            sended: 0,
        }
    }

    fn set(&mut self, reg: char, val: i64) {
        self.reg[(reg as u8 - b'a') as usize] = val;
    }

    fn get(&self, reg: char) -> i64 {
        self.reg[(reg as u8 - b'a') as usize]
    }

    fn pop(&mut self) -> Option<i64> {
        self.sounds.pop_front()
    }

    fn receive(&mut self, val: i64) {
        if let Some(reg) = self.rec_reg {
            self.set(reg, val);
        }
        self.rec_reg = None;
    }

    pub fn step(&mut self) -> bool {
        if self.ip >= self.asm.len() {
            return false;
        }
        self.ip += 1;

        match &self.asm[self.ip-1] {
            Instr::Snd(s) => {
                self.sounds.push_back(s.value(&self.reg));
                self.sended += 1;
            },
            Instr::Rcv(reg) => {
                if self.get(*reg) != 0 { 
                    if let Some(&frq) = self.sounds.back() {
                        self.frq = Some(frq);
                    }
                }
                self.rec_reg = Some(*reg);
            },
            Instr::Set(x,y) => self.set(*x, y.value(&self.reg)),
            Instr::Add(x,y) => self.set(*x, self.get(*x) + y.value(&self.reg)),
            Instr::Sub(x,y) => self.set(*x, self.get(*x) - y.value(&self.reg)),
            Instr::Mul(x,y) => self.set(*x, self.get(*x) * y.value(&self.reg)),
            Instr::Mod(x,y) => self.set(*x, self.get(*x) % y.value(&self.reg)),
            Instr::Jgz(x,y) => if x.value(&self.reg) > 0 {
                self.ip = (self.ip as i64 + y.value(&self.reg)) as usize - 1;
            },
            Instr::Jnz(x,y) => if x.value(&self.reg) != 0 {
                self.ip = (self.ip as i64 + y.value(&self.reg)) as usize - 1;
            },
        }

        true
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

fn part1(vm: &mut VM) -> i64 {
    while vm.frq.is_none() {
        vm.step();
    }

    vm.frq.unwrap()
}

fn part2(vm: &VM) -> i64 {
    let mut vm0 = vm.clone();
    let mut vm1 = vm.clone();
    vm1.reg[(b'p' - b'a') as usize] = 1;
    let mut stop = false;

    while !stop {
        while vm0.rec_reg.is_none() {
            if !vm0.step() {
                break;
            }
        }

        while vm1.rec_reg.is_none() {
            if !vm1.step() {
                break;
            }
        }

        stop = true;
        if let Some(val) = vm1.pop() {
            vm0.receive(val);
            stop = false;
        }
        if let Some (val) = vm0.pop() {
            vm1.receive(val);
            stop = false;
        }
    }

    vm1.sended
}

fn main() {
    let vm = VM::new(&std::fs::read_to_string("../input/18").unwrap());

    println!("Part 1: {}", part1(&mut vm.clone()));
    println!("Part 2: {}", part2(&vm));
}
