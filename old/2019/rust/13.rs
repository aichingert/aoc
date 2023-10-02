#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N};

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

fn main() {
    let opcodes = std::fs::read_to_string("../input/13").unwrap().trim()
        .split(',')
        .map(|n| n.parse::<N>().unwrap())
        .collect::<Vec<N>>();


    println!("Part one: {}", part_one(&opcodes));
    
}
