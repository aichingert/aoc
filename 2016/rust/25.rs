// Advent of Code 2016, day 25
// (c) aichingert

#[path="asm.rs"] mod asm;
use asm::Runner;
use std::collections::HashMap;

fn part1(inp: &String) -> i32 {
    let mut runner = Runner::new(inp);
    let mut a: i32 = 0;

    'part1: loop {
        a += 1;

        runner.reg = HashMap::from([("a",a),("b",0),("c",0),("d",0)]);
        runner.out.clear();
        runner.exec("a");

        for i in 0..runner.out.len() {
            match i & 1 == 0 {
                true => if runner.out[i] == 1 { continue 'part1; },
                false => if runner.out[i] == 0 { continue 'part1; },
            };
        }

        break
    }

    a
}

fn part2<'a>() -> &'a str {
    "Merry christmas!"
}

fn main() {
    let inp = std::fs::read_to_string("../input/25").unwrap();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2());
}
