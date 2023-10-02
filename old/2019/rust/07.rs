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
            VM::new(opcodes, perm[0]),
            VM::new(opcodes, perm[1]),
            VM::new(opcodes, perm[2]),
            VM::new(opcodes, perm[3]),
            VM::new(opcodes, perm[4])
        ];
        let mut input = 0;

        for i in 0..cpus.len() {
            loop {
                match cpus[i].exec() {
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

fn part_two(opcodes: &Vec<N>) -> N {
    let mut signal = 0;
    let perms = permutations(5, &mut vec![5,6,7,8,9]);

    for perm in perms.iter() {
        let mut cpus = vec![
            VM::new(opcodes, perm[0]),
            VM::new(opcodes, perm[1]),
            VM::new(opcodes, perm[2]),
            VM::new(opcodes, perm[3]),
            VM::new(opcodes, perm[4])
        ];

        for i in 0..cpus.len() {
            loop {
                match cpus[i].exec() {
                    Status::Input => {
                        break;
                    }
                    _ => {},
                };
            }
        }

        let mut status = Status::Normal;
        let (mut input, mut i) = (0, 0);

        while status != Status::Exit {
            loop {
                if cpus[i]._get_next_opcode() == 3 {
                    cpus[i]._set_input(input);
                }

                status = cpus[i].exec();

                match status {
                    Status::Output(n) => {
                        input = n;
                        break;
                    }
                    Status::Exit => break,
                    _ => {},
                };
            }

            i = (i + 1) % cpus.len();
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
    println!("Part 2: {}", part_two(&opcodes));
}
