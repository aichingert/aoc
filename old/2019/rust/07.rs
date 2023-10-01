// Advent of Code 2019, day 7
// (c) aichingert

#[path="intcode.rs"] mod intcode;
#[path="../../utils/rust/permutations.rs"] mod permutations;
use intcode::{VM, Status, N};
use permutations::permutations;

fn part_one(opcodes: &Vec<N>) -> N {
    let mut signal = 0;
    let perms = permutations(5, &mut vec![0,1,2,3,4]);

    for perm in perms.iter() {
        let mut cpus = vec![
            VM::new(opcodes.clone(), perm[0]),
            VM::new(opcodes.clone(), perm[1]),
            VM::new(opcodes.clone(), perm[2]),
            VM::new(opcodes.clone(), perm[3]),
            VM::new(opcodes.clone(), perm[4])
        ];
        let mut input = 0;

        for i in 0..cpus.len() {
            loop {
                match cpus[i].execute() {
                    Status::Input => cpus[i]._set_input(input),
                    Status::Output(n) => input = n,
                    Status::Exit => break,
                    _ => {},
                }
            }
        }

        signal = signal.max(input);
    }

    signal
}


fn main() {
    let opcodes = std::fs::read_to_string("../input/07").unwrap().trim()
        .split(',')
        .map(|n| n.parse::<N>().unwrap())
        .collect::<Vec<N>>();

    println!("Part 1: {}", part_one(&opcodes));
}
