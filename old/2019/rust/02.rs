// Advent of Code 2019, day 2
// (c) aichingert

#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};

fn part_one(opcodes: &Vec<N>) -> N {
    let mut vm: VM = VM::new(opcodes, 0);

    loop {
        match vm.exec() {
            Status::Exit => break,
            _ => {},
        };
    }

    vm._get_position(0)
}

fn part_two(opcodes: &mut Vec<N>) -> N {
    for noun in 0..100 {
        for verb in 0..100 {
            opcodes[1] = noun;
            opcodes[2] = verb;

            let mut vm: VM = VM::new(opcodes, 0);

            loop {
                match vm.exec() {
                    Status::Exit => break,
                    _ => {},
                }
            }

            if vm._get_position(0) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("not found")
}

fn main() {
    let mut opcodes = read_input(2);
    opcodes[1] = 12;
    opcodes[2] = 2;

    println!("Part 1: {}", part_one(&opcodes));
    println!("Part 2: {}", part_two(&mut opcodes));
}
