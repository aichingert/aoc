// Advent of Code 2018, day 16
// (c) aichingert

use std::collections::{HashSet,HashMap};

const OPCODES: [Opcode;16] = [Opcode::Addr,Opcode::Addi,Opcode::Mulr,Opcode::Muli,Opcode::Banr,
                    Opcode::Bani,Opcode::Borr,Opcode::Bori,Opcode::Setr,Opcode::Seti,Opcode::Gtir,
                    Opcode::Gtri,Opcode::Gtrr,Opcode::Eqir,Opcode::Eqri,Opcode::Eqrr];
#[derive(Debug,Clone,Copy)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Opcode {
    fn in_bounds(n: i32) -> bool {
        n < 4 || n > -1
    }

    fn execute(&self, cur: &mut [i32], expected: &[i32], a: i32, b: i32, c: i32) -> bool {
        if !Opcode::in_bounds(c) {return false;}

        match self {
            Opcode::Addr => {
                if !Opcode::in_bounds(a) || !Opcode::in_bounds(b) { return false; }
                cur[c as usize] = cur[a as usize] + cur[b as usize];
            },
            Opcode::Addi => {
                if !Opcode::in_bounds(a) { return false; }
                cur[c as usize] = cur[a as usize] + b;
            },
            Opcode::Mulr => {
                if !Opcode::in_bounds(a) || !Opcode::in_bounds(b) { return false; }
                cur[c as usize] = cur[a as usize] * cur[b as usize];
            },
            Opcode::Muli => {
                if !Opcode::in_bounds(b) { return false; }
                cur[c as usize] = cur[a as usize] * b;
            },
            Opcode::Banr => {
                if !Opcode::in_bounds(a) || !Opcode::in_bounds(b) { return false; }
                cur[c as usize] = cur[a as usize] & cur[b as usize];
            },
            Opcode::Bani => {
                if !Opcode::in_bounds(b) { return false; }
                cur[c as usize] = cur[a as usize] & b;
            },
            Opcode::Borr => {
                if !Opcode::in_bounds(a) || !Opcode::in_bounds(b) { return false; }
                cur[c as usize] = cur[a as usize] | cur[b as usize];
            },
            Opcode::Bori => {
                if !Opcode::in_bounds(a) { return false; }
                cur[c as usize] = cur[a as usize] | b;
            },
            Opcode::Setr => {
                if !Opcode::in_bounds(a) { return false; }
                cur[c as usize] = cur[a as usize];
            },
            Opcode::Seti => cur[c as usize] = a,
            Opcode::Gtir => {
                if !Opcode::in_bounds(b) {return false;}
                cur[c as usize] = if a > cur[b as usize] { 1 } else { 0 };
            },
            Opcode::Gtri => {
                if !Opcode::in_bounds(a) {return false;}
                cur[c as usize] = if cur[a as usize] > b { 1 } else { 0 };
            },
            Opcode::Gtrr => {
                if !Opcode::in_bounds(a) || !Opcode::in_bounds(b) { return false; }
                cur[c as usize] = if cur[a as usize] > cur[b as usize] { 1 } else { 0 };
            },
            Opcode::Eqir => {
                if !Opcode::in_bounds(b) {return false;}
                cur[c as usize] = if a == cur[b as usize] { 1 } else { 0 };
            },
            Opcode::Eqri => {
                if !Opcode::in_bounds(a) {return false;}
                cur[c as usize] = if cur[a as usize] == b { 1 } else { 0 };
            },
            Opcode::Eqrr => {
                if !Opcode::in_bounds(a) || !Opcode::in_bounds(b) { return false; }
                cur[c as usize] = if cur[a as usize] == cur[b as usize] { 1 } else { 0 };
            },
        }

        &*cur == expected
    }
}

fn part1() -> i32 {
    let mut ans = 0;
    let inp = std::fs::read_to_string("../input/16").unwrap();
    let (inp,exmpl) = inp.split_once("\n\n\n\n").unwrap();
    let inp = inp.split("\n\n").collect::<Vec<&str>>();
    let mut known: HashSet<usize> = HashSet::new();
    let mut mapping: HashMap<i32, Opcode> = HashMap::new();

    while known.len() < 16 {
        for i in 0..inp.len() {
            let lines = inp[i].lines().collect::<Vec<&str>>();
            let (_,arr) = lines[0].split_once('[').unwrap();
            let cur = arr[..arr.len()-1]
                .split(", ")
                .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let (_,arr) = lines[2].split_once('[').unwrap();
            let exp = arr[..arr.len()-1]
                .split(", ")
                .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            let abc = lines[1].split(' ').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            let mut count = 0;
            let mut codes = Vec::<usize>::new();

            for opcode in 0..OPCODES.len() {
                let mut this = cur.clone();
                if OPCODES[opcode].execute(&mut this, &exp, abc[1], abc[2], abc[3]) 
                    && !known.contains(&opcode) {
                    codes.push(opcode);
                }
            }


            if codes.len() == 1 {
                known.insert(codes[0]);
                mapping.insert(abc[0], OPCODES[codes[0]]);
            }
        }
    }

    let mut reg = [0,0,0,0];

    for l in exmpl.lines() {
        let r = l.split(' ').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        mapping[&r[0]].execute(&mut reg, &[], r[1],r[2],r[3]);
    }

    reg[0]
}

fn main() {
    println!("Part 1: {}", part1());
}
