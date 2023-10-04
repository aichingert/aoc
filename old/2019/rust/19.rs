#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};

fn part_one(opcodes: &Vec<N>) -> N {
    let mut ans = 0;

    for i in 0..50 {
        for j in 0..50 {
            let mut vm = VM::new(&opcodes, i);
            let mut first = true;

            loop {
                if vm._get_next_opcode() == 3 {
                    if first {
                        first = false;
                    } else {
                        vm._set_input(j);
                    }
                }

                match vm.exec() {
                    Status::Output(n) => ans += n,
                    Status::Exit => break,
                    _ => {},
                };
            }
        }
    }

    ans
}

fn main() {
    let opcodes = read_input(19);

    println!("Part one: {}", part_one(&opcodes));
}
