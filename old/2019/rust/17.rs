#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};
use std::collections::HashSet;

const NEW_LINE: N = 10;
const SCAFFOLD: N = 35;

fn part_one(opcodes: &Vec<N>) -> N {
    let mut vm = VM::new(&opcodes, 0);

    let (mut r, mut c) = (0, 0);
    let mut scaffolds = HashSet::new();
    let mut ans = 0;
    let mut dir = (0, 0);

    loop {
        match vm.exec() {
            Status::Output(n) => {
                match n {
                    NEW_LINE => { r += 1; c = 0; }
                    SCAFFOLD => { scaffolds.insert((r, c)); c += 1; }
                    ch => {
                        match ch {
                            60 => dir = (0,-1),
                            62 => dir = (0, 1),
                            94 => dir = (-1,0),
                            118 => dir = (1,0),
                            _ => (),
                        };

                        c += 1;
                    }
                };
            }
            Status::Exit => break,
            _ =>(),
        }
    }

    let start = (0, 0);



    println!("{:?}", dir);

    for (i, j) in scaffolds.iter() {
        if scaffolds.contains(&(*i + 1, *j)) 
            && scaffolds.contains(&(*i - 1, *j)) 
            && scaffolds.contains(&(*i, *j + 1)) 
            && scaffolds.contains(&(*i, *j - 1)) 
        {
            ans += i * j;
        }
    }

    for i in 0..50 {
        for j in 0..50 {
            if scaffolds.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    
    ans
}

fn main() {
    let mut opcodes = read_input(17);
    
    println!("Part one: {}", part_one(&opcodes));
}
