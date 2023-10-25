#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};

fn part_one(opcodes: &Vec<N>) -> N {
    let mut vm = VM::new(opcodes, 0);
    let commands = "\
    NOT A J\n\
    NOT B T\n\
    OR J T\n\
    NOT C J\n\
    OR J T\n\
    OR T J\n\
    AND D J\n\
    AND T J\n\
    WALK\n
    ".chars().map(|ch| (ch as u8) as N).collect::<Vec<N>>();
    let mut cur = 0;
    let mut ans = 0;

    loop {
        if vm._get_next_opcode() == 3 {
            vm._set_input(commands[cur] as N);
            cur += 1;
        }

        match vm.exec() { 
            Status::Output(n) => ans = n,
            Status::Exit => break,
            _ => (),
        }
    }

    ans
}

fn main() {
    let opcodes = read_input(21);
    
    println!("Part one: {}", part_one(&opcodes));
}
