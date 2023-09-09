// Advent of Code 2019, day 7
// (c) aichingert

#[path="intcode.rs"] mod intcode;
#[path="../../utils/rust/permutations.rs"] mod permutations;
use intcode::Computer;
use permutations::permutations;

fn part1(opcodes: &Vec<i32>) -> i32 {
    let mut signal = 0i32;
    let mut perms = permutations(5, &mut vec![0,1,2,3,4]);

    for perm in perms.iter() {
        let mut amp_a = Computer::new(opcodes.clone(), vec![perm[0], 0]);
        amp_a.run();
        //println!("{:?}", amp_a.output);
        let mut amp_b = Computer::new(opcodes.clone(), vec![perm[1], amp_a.output[0]]);
        amp_b.run();
        //println!("{:?}", amp_b.output);
        let mut amp_c = Computer::new(opcodes.clone(), vec![perm[2], amp_b.output[0]]);
        amp_c.run();
        //println!("{:?}", amp_c.output);
        let mut amp_d = Computer::new(opcodes.clone(), vec![perm[3], amp_c.output[0]]);
        amp_d.run();
        //println!("{:?}", amp_d.output);
        let mut amp_e = Computer::new(opcodes.clone(), vec![perm[4], amp_d.output[0]]);
        amp_e.run();
        //println!("{:?}", amp_e.output);
        //println!("{:?} - {:?}", perm, amp_e.output);
        signal = signal.max(amp_e.output[0]);
    }

    signal
}

fn part2() {}

fn main() {
    let opcodes = std::fs::read_to_string("../input/07").unwrap().trim().split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    println!("Part 1: {}", part1(&opcodes));
    //println!("Part 2: {}", part2());
}
