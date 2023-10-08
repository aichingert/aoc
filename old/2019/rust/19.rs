#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};
use std::collections::HashSet;

fn part_one(opcodes: &Vec<N>) -> N {
    let mut ans = 0;

    for i in 0..50 {
        for j in 0..50 {
            let mut vm = VM::new(&opcodes, i);
            let mut first = true;

            loop {
                if vm._get_next_opcode() == 3 {
                    if first {
                        first = false;
                    } else {
                        vm._set_input(j);
                    }
                }

                match vm.exec() {
                    Status::Output(n) => ans += n,
                    Status::Exit => break,
                    _ => {},
                };
            }
        }
    }

    ans
}

fn part_two(opcodes: &Vec<N>) -> N {
    let mut beam = HashSet::new();

    for i in 800..1200 {
        for j in 800..1200 {
            let mut vm = VM::new(opcodes, i);
            let mut fs = true;
            
            loop {
                if vm._get_next_opcode() == 3 {
                    if fs {
                        fs = false;
                    } else {
                        vm._set_input(j);
                    }
                }

                match vm.exec() {
                    Status::Output(n) => if n == 1 { beam.insert((i, j)); },
                    Status::Exit => break,
                    _ => {},
                };
            }
        }
    }

    for i in 800..1100 {
        'outer: for j in 800..1100 {
            for r in i..i+100 {
                for c in j..j+100 {
                    if !beam.contains(&(r, c)) {
                        continue 'outer;
                    }
                }
            }

            return i * 10000 + j;
        }
    }

    panic!("no solution found");
}

fn main() {
    let opcodes = read_input(19);

    println!("Part one: {}", part_one(&opcodes));
    println!("Part two: {}", part_two(&opcodes));
}

