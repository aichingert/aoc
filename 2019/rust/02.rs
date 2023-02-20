// Advent of Code 2019, day 2
// (c) aichingert

#[path="intcode.rs"] mod intcode;
use intcode::Computer;

fn part1(opcodes: &Vec<i32>) -> i32 {
    let mut computer: Computer = Computer::new(opcodes.clone(), 0);

    computer.run();
    computer.opcodes[0]
}

fn part2(opcodes: &mut Vec<i32>) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            opcodes[1] = noun;
            opcodes[2] = verb;

            let mut computer: Computer = Computer::new(opcodes.clone(), 0);
            
            computer.run();
            
            if computer.opcodes[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("noun or verb is too small!");
}

fn main() {
    let mut opcodes = std::fs::read_to_string("../input/02").unwrap().trim().split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    opcodes[1] = 12;
    opcodes[2] = 1;

    println!("Part 1: {}", part1(&opcodes));
    println!("Part 2: {}", part2(&mut opcodes));
}
