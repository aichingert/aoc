#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N};
use std::collections::HashSet;

fn main() {
    let opcodes = std::fs::read_to_string("../input/11").unwrap().trim()
        .split(',')
        .map(|n| n.parse::<N>().unwrap())
        .collect::<Vec<N>>();

    let mut pos = (0, 0);
    let mut dir = (-1, 0);

    let mut map: HashSet<(N, N)> = HashSet::new();
    let mut seen = HashSet::from([pos]);

    let mut vm = VM::new(&opcodes, 0);
    let mut status = Status::Normal;

    while status != Status::Exit {
        let mut first = true;
        let (mut paint, mut turn) = (0, 0);
        
        vm._set_input(if map.contains(&pos) { 1 } else { 0 });
        loop {
            status = vm.exec();

            match status {
                Status::Output(n) => if first {
                    first = false;
                    paint = n;
                } else {
                    turn = n;
                    break;
                }
                Status::Exit => break,
                _ => {},
            }
        }

        dir = match turn {
            0 => match dir {
                (-1, 0) => (0, -1),
                (0, -1) => (1, 0),
                (1, 0)  => (0, 1),
                (0, 1)  => (-1, 0),
                _ => unreachable!(),
            }
            1 => match dir {
                (-1, 0)  => (0, 1),
                (0, 1)  => (1, 0),
                (1, 0)  => (0, -1),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            }
            _ => panic!("invalid turn"),
        };

        match paint {
            0 => map.remove(&pos),
            1 => map.insert(pos),
            _ => panic!("invalid paint"),
        };

        pos = (pos.0 + dir.0, pos.1 + dir.1);

        if status != Status::Exit {
            seen.insert(pos); 
        }
    }
    
    println!("Part one: {}", seen.len());
}
