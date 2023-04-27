// Advent of Code 2018, day 16
// (c) aichingert

use std::collections::{HashSet,HashMap};

const OPCODES: [Opcode;16] = [Opcode::Addr,Opcode::Addi,Opcode::Mulr,Opcode::Muli,Opcode::Banr,
                    Opcode::Bani,Opcode::Borr,Opcode::Bori,Opcode::Setr,Opcode::Seti,Opcode::Gtir,
                    Opcode::Gtri,Opcode::Gtrr,Opcode::Eqir,Opcode::Eqri,Opcode::Eqrr];

struct Inp {
    before: Vec<i32>,
    after: Vec<i32>,
    codes: Vec<i32>,
}

impl Inp {
    fn new(before: Vec<i32>, after: Vec<i32>, codes: Vec<i32>) -> Self {
        Self { before, after, codes }
    }
}

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

fn part1(inps: &Vec<Inp>) -> i32 {
    let mut ans = 0;

    for i in 0..inps.len() {
        let mut count = 0;

        for j in 0..OPCODES.len() {
            if OPCODES[j].execute(
                &mut inps[i].before.clone(), 
                &inps[i].after, 
                inps[i].codes[1], inps[i].codes[2],inps[i].codes[3]
            ) {
                count += 1;
            }
        }

        if count > 2 {
            ans += 1;
        }
    }

    ans 
}

fn part2(inps: &Vec<Inp>, example: &String) -> i32 {
    let mut known = HashSet::<usize>::new();
    let mut mapping = HashMap::<i32, Opcode>::new();

    while known.len() < 16 {
        for i in 0..inps.len() {
            let mut codes = Vec::<usize>::new();

            for opcode in 0..OPCODES.len() {
                if OPCODES[opcode].execute(
                    &mut inps[i].before.clone(), 
                    &inps[i].after, 
                    inps[i].codes[1], inps[i].codes[2],inps[i].codes[3]
                ) && !known.contains(&opcode) {
                    codes.push(opcode);
                }
            }

            if codes.len() == 1 { 
                known.insert(codes[0]); 
                mapping.insert(inps[i].codes[0], OPCODES[codes[0]]);
            }
        }

    }

    let mut reg = [0,0,0,0];

    for l in example.lines() {
        let r = l.split(' ').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        mapping[&r[0]].execute(&mut reg, &[], r[1],r[2],r[3]);
    }

    reg[0]
}

fn parse() -> (Vec<Inp>, String) {
    let mut inps = Vec::<Inp>::new();
    let inp = std::fs::read_to_string("../input/16").unwrap();
    let (inp,example) = inp.split_once("\n\n\n\n").unwrap();

    let inp = inp.split("\n\n").collect::<Vec<&str>>();

    for i in 0..inp.len() {
        let lines = inp[i].lines().collect::<Vec<&str>>();
        let mut regs = Vec::new();

        for j in (0..3).step_by(2) {
            let (_,arr) = lines[j].split_once('[').unwrap();

            regs.push(arr[..arr.len()-1]
                .split(", ")
                .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>());
        }

        inps.push(Inp::new(
                regs[0].clone(), regs[1].clone(), 
                lines[1].split(' ').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>())
        );
    }


    (inps, example.to_string())
}

fn main() {
    let (inps, example) = parse();

    println!("Part 1: {}", part1(&inps));
    println!("Part 2: {}", part2(&inps, &example));
}
