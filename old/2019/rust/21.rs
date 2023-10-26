#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};

fn solve(opcodes: &Vec<N>, commands: Vec<N>) -> N {
    let mut vm = VM::new(opcodes, 0);
    
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

    let part_one = "\
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
    
    let part_two = "\
    NOT A J\n\
    NOT B T\n\
    OR J T\n\
    NOT C J\n\
    OR J T\n\
    OR T J\n\
    AND E J\n\
    OR H J\n\
    AND J T\n\
    OR T J\n\
    AND D J\n\
    AND T J\n\
    RUN\n
    ".chars().map(|ch| (ch as u8) as N).collect::<Vec<N>>();

    println!("Part one: {}", solve(&opcodes, part_one));
    println!("Part one: {}", solve(&opcodes, part_two));
}
