#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};

fn part_one(opcodes: &Vec<N>) -> N {
    let mut vm = VM::new(opcodes, 0);
    let mut status = Status::Normal;
    let mut tiles = 0;
    
    while status != Status::Exit {
        let mut out = Vec::<N>::new();

        loop {
            status = vm.exec();

            match status {
                Status::Output(n) => match out.len() == 2 {
                    true => {
                        out.push(n);
                        break;
                    }
                    false => out.push(n),
                },
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
    let mut vm = VM::new(opcodes, -1);

    let mut status = Status::Normal;
    let mut tiles = 0;
    
    while status != Status::Exit {
        let mut out = Vec::<N>::new();

        loop {
            status = vm.exec();

            match status {
                Status::Output(n) => match out.len() == 2 {
                    true => {
                        out.push(n);
                        break;
                    }
                    false => out.push(n),
                },
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

fn main() {
    let mut opcodes = read_input(13);

    println!("Part one: {}", part_one(&opcodes));
    println!("Part two: {}", part_two(&mut opcodes));
}
