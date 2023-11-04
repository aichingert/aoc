use std::collections::HashSet;

#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};

const MACHINES: usize = 50;

struct Nic {
    vm: VM,
    packets: Vec<(bool, N, N)>
}

struct Nat {
    mem: HashSet<(N, N)>,
    last_packet: Option<(N, N)>,
}

impl Nic {
    fn new(opcodes: &Vec<N>) -> Self {
        Self {
            vm: VM::new(opcodes, 0),
            packets: Vec::new(),
        }
    }
}

impl Nat {
    fn new() -> Self {
        Self {
            mem: HashSet::new(),
            last_packet: None,
        }
    }
}

fn solve(opcodes: &Vec<N>) -> (N, N) {
    let mut part_one: N = -1;
    let mut nics: [Nic; MACHINES] = core::array::from_fn(|_| Nic::new(opcodes));
    let mut nat : Nat = Nat::new();

    for i in 0..MACHINES {
        nics[i].vm._set_input(i as N);

        loop {
            if nics[i].vm._get_next_opcode() == 3 {
                _ = nics[i].vm.exec();
                break;
            }
            
            _ = nics[i].vm.exec();
        }
    }

    loop {
        let mut p = false;
        for i in 0..MACHINES {
            let (mut f, mut sb) = (true, false);

            loop {
                if nics[i].vm._get_next_opcode() == 3 {
                    nics[i].vm._set_input(if nics[i].packets.len() == 0 {
                        if !f { sb = true; }
                        f = false;
                        -1
                    } else {
                        if nics[i].packets[0].0 {
                            let r = nics[i].packets.remove(0);
                            r.2
                        } else {
                            nics[i].packets[0].0 = true;
                            nics[i].packets[0].1 
                        }
                    });
                }

                match nics[i].vm.exec() {
                    Status::Output(n) => {
                        let (addr, mut vals) = (n, vec![]);

                        loop {
                            match nics[i].vm.exec() {
                                Status::Output(n) => {
                                    vals.push(n);
                                    if vals.len() == 2 { break; }
                                }
                                _ => (),
                            }
                        }

                        p = true;
                        if addr == 255 {
                            if part_one == -1 { part_one = vals[1]; }
                            nat.last_packet = Some((vals[0], vals[1]));
                        } else {
                            nics[addr as usize].packets.push((false, vals[0], vals[1]));
                        }
                    }
                    Status::Exit => break,
                    _ => (),
                }

                if sb {
                    break;
                }
            }
        }

        if !p {
            if let Some(packet) = nat.last_packet {
                if !nat.mem.insert(packet) {
                    return (part_one, packet.1);
                }

                nics[0].packets.push((false, packet.0, packet.1));
            } else {
                panic!("no one send a packet to nat");
            }
        }
    }
}

fn main() {
    let opcodes = read_input(23);
    let (part_one, part_two) = solve(&opcodes);
    
    println!("Part one: {}", part_one);
    println!("Part one: {}", part_two);
}
