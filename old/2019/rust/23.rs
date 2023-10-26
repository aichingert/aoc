#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};

const MACHINES: usize = 50;

struct Nic {
    vm: VM,
    packets: Vec<(N, N)>
}

impl Nic {
    fn new(opcodes: &Vec<N>) -> Self {
        Self {
            vm: VM::new(opcodes, 0),
            packets: Vec::new(),
        }
    }
}

fn part_one(opcodes: &Vec<N>) -> N {
    let mut nics: [Nic; MACHINES] = core::array::from_fn(|_| Nic::new(opcodes));
    let mut ans = 0;

    for i in 0..MACHINES {
        nics[i].vm._set_input(i as N);
        let mut done = false;

        loop {
            if nics[i].vm._get_next_opcode() == 3 {
                if done { break; }
                done = true;
            }
            
            match nics[i].vm.exec() {
                _ => (),
            }
        }
    }



    loop {
        if nics[cur].vm._get_next_opcode() == 3 {
            nics[cur].vm._set_input(-1);
        }

        match nics[cur].vm.exec() { 
            Status::Output(n) => println!("{}", n),
            Status::Exit => break,
            _ => (),
        }

    }

    ans
}

fn main() {
    let opcodes = read_input(23);
    
    println!("Part one: {}", part_one(&opcodes));
}
