// Advent of Code 2017, day 23
// (c) aichingert

#[allow(unused)]
#[path="d18.rs"] mod d18;
use d18::{VM,Instr};

fn part1(vm: &mut VM) -> u32 {
    let mut count = 0;

    while vm.step() {
        if vm.ip >= vm.asm.len() { break; }
        match vm.asm[vm.ip] {
            Instr::Mul(_,_) => count += 1,
            _ => (),
        }
    }

    count
}

fn part2() -> i64 {
    let lower = 5700 + 100_000;
    let upper = 5700 + 117_000;
    let mut ans = 0;

    for check in (lower..=upper).step_by(17) {
        if !is_prime(check) {
            ans += 1;
        }
    }
    
    ans
}

fn is_prime(n: i64) -> bool {
    for i in 2..(n as f64).sqrt() as i64 {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

fn main() {
    let mut vm = VM::new(&std::fs::read_to_string("../input/23").unwrap());

    println!("Part 1: {}", part1(&mut vm));
    println!("Part 2: {}", part2());
}
