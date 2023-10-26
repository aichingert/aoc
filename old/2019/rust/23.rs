#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};

const MACHINES: usize = 50;

struct Nic {
    vm: VM,
    packets: Vec<(bool, N, N)>
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
    let mut cur = 0;

    for i in 0..MACHINES {
        nics[i].vm._set_input(i as N);
        let mut done = false;

        loop {
            if nics[i].vm._get_next_opcode() == 3 {
                if done { break; }
                done = true;
            }
            
            match nics[i].vm.exec() {
                Status::Output(n) => println!("FIR: {}", n),
                _ => (),
            }
        }
    }

    loop {
        for i in 0..MACHINES {
            let mut first = true;
            let mut should_break = false;

            loop {
                if nics[i].vm._get_next_opcode() == 3 {
                    println!("INP: {i}");
                    nics[i].vm._set_input(if nics[i].packets.len() == 0 {
                        if !first { should_break = true; }
                        first = false;
                        -1
                    } else {
                        if nics[i].packets[0].0 {
                            let rem = nics[i].packets.remove(0);
                            rem.2
                        } else {
                            nics[i].packets[0].0 = true;
                            nics[i].packets[0].1 
                        }
                    });
                }

                match nics[i].vm.exec() {
                    Status::Output(n) => {
                        let addr = n;
                        let mut vals = vec![];

                        loop {
                            match nics[i].vm.exec() {
                                Status::Output(n) => {
                                    vals.push(n);

                                    if vals.len() == 2 {
                                        break;
                                    }
                                }
                                _ => (),
                            }
                        }

                        if addr == 255 {
                            return vals[1];
                        } else {
                            nics[addr as usize].packets.push((false, vals[0], vals[1]));
                        }

                        println!("Addr: {addr} - X: {}; Y: {}", vals[0], vals[1]);
                    }
                    Status::Exit => break,
                    _ => (),
                }

                if should_break {
                    break;
                }
            }
            ans = 0;
        }
    }

    ans
}

fn main() {
    let opcodes = read_input(23);
    
    println!("Part one: {}", part_one(&opcodes));
}
