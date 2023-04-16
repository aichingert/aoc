// Advent of Code 2017, day 18
// (c) aichingert

pub struct VM {
    pub reg: Vec<i64>,
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
            reg: vec![0;26],
            asm,
            ip: 0,
            sounds: Vec::new(),
            rec: None,
        }
    }

    fn set(&mut self, reg: char, val: i64) {
        self.reg[(reg as u8 - b'a') as usize] = val;
    }

    fn get(&self, reg: char) -> i64 {
        self.reg[(reg as u8 - b'a') as usize]
    }

    pub fn step(&mut self) -> bool {
        if self.ip >= self.asm.len() {
            return false;
        }
        self.ip += 1;

        match &self.asm[self.ip-1] {
            Instr::Snd(s) => self.sounds.push(s.value(&self.reg)),
            Instr::Rcv(x) => if x.value(&self.reg) != 0 { 
                self.rec = Some(self.sounds[self.sounds.len()-1]); 
            },
            Instr::Set(x,y) => self.set(*x, y.value(&self.reg)),
            Instr::Add(x,y) => self.set(*x, self.get(*x) + y.value(&self.reg)),
            Instr::Mul(x,y) => self.set(*x, self.get(*x) * y.value(&self.reg)),
            Instr::Mod(x,y) => self.set(*x, self.get(*x) % y.value(&self.reg)),
            Instr::Jgz(x,y) => if x.value(&self.reg) > 0 {
                self.ip = (self.ip as i64 + y.value(&self.reg)) as usize - 1;
            }
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
    while vm.rec == None {
        vm.step();
    }

    vm.rec.unwrap()
}

fn main() {
    let mut vm = VM::new(&std::fs::read_to_string("../input/18").unwrap());

    println!("Part 1: {}", part1(&mut vm));
}
