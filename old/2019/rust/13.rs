#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};

use std::collections::HashSet;
use std::cmp::Ordering;

const Y: usize = 19;
const X: usize = 37;

fn part_one(opcodes: &Vec<N>) -> N {
    let mut vm = VM::new(opcodes, 0);
    let mut status = Status::Normal;
    let mut tiles = 0;
    
    while status != Status::Exit {
        let mut out = Vec::<N>::new();

        loop {
            status = vm.exec();

            match status {
                Status::Output(n) => {
                    out.push(n);
                    if out.len() > 2 { break; }
                }
                Status::Exit => break,
                _ => {},
            }
        }

        if out.len() > 1 && out[2] == 2  {
            tiles += 1;
        }
    }

    tiles
}

fn part_two(opcodes: &mut Vec<N>) -> N {
    opcodes[0] = 2;

    let mut vm = VM::new(opcodes, 0);
    let mut blocks: HashSet<(N, N)> = HashSet::new();
    let mut paddle: (N, N) = (0, 0);
    let mut ball: (N, N) = (0, 0);

    for _ in 0..(Y + 1) * (X + 1) { 
        let mut out = Vec::<N>::new();

        loop {
            match vm.exec() {
                Status::Output(n) => {
                    out.push(n);
                    if out.len() > 2 { break; }
                }
                _ => {},
            }
        }

        match out[2] {
            2 => { blocks.insert((out[1], out[0])); }
            3 => paddle = (out[1], out[0]),
            4 => ball = (out[1], out[0]),
            _ => (),
        };
    }

    'outer: while blocks.len() > 0 {
        let mut out = Vec::<N>::new();

        loop {
            match vm.exec() {
                Status::Output(n) => { 
                    out.push(n);
                    if out.len() > 2 { break; }
                }
                Status::Exit => break 'outer,
                _ => (),
            }
        }

        let pt = (out[1], out[0]);

        match out[2] {
            0 => if blocks.contains(&pt) { blocks.remove(&pt); },
            3 => paddle = pt,
            4 => ball = pt,
            _ => (),
        };

        vm._set_input(match ball.1.cmp(&paddle.1) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        });
    }

    let mut score = 0;
    while score <= 0 {
        match vm.exec() {
            Status::Output(n) => score = n,
            _ => (),
        };
    }
    
    score
}

fn main() {
    let mut opcodes = read_input(13);

    println!("Part one: {}", part_one(&opcodes));
    println!("Part two: {}", part_two(&mut opcodes));
}
