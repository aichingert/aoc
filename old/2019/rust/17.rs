#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};
use std::collections::HashSet;

const NEW_LINE: N = 10;
const SCAFFOLD: N = 35;

fn part_one(opcodes: &Vec<N>) -> (N, (N,N),(N,N), HashSet<(N, N)>) {
    let mut vm = VM::new(&opcodes, 0);

    let (mut r, mut c) = (0, 0);
    let mut scaffolds = HashSet::new();
    let mut ans = 0;
    let mut dir = (0, 0);
    let mut start = (0, 0);

    loop {
        match vm.exec() {
            Status::Output(n) => {
                match n {
                    NEW_LINE => { r += 1; c = 0; }
                    SCAFFOLD => { scaffolds.insert((r, c)); c += 1; }
                    ch => {
                        match ch {
                            60 => { start = (r,c); dir = (0,-1); },
                            62 => { start = (r,c); dir = (0, 1); },
                            94 => { start = (r,c); dir = (-1,0); },
                            118 =>{ start = (r,c); dir = (1, 0); },
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

    for (i, j) in scaffolds.iter() {
        if scaffolds.contains(&(*i + 1, *j)) 
            && scaffolds.contains(&(*i - 1, *j)) 
            && scaffolds.contains(&(*i, *j + 1)) 
            && scaffolds.contains(&(*i, *j - 1)) 
        {
            ans += i * j;
        }
    }
    
    (ans, start, dir, scaffolds)
}

/*
fn get_next_position(cur: (N, N), s: &HashSet<(N, N)>, v: &HashSet<(N, N)>) -> Option<(N,N)> {
    for vec in [(0,1),(1,0),(0,-1),(-1,0)] {
        let loc = (cur.0 + vec.0, cur.1 + vec.1);

        if s.contains(&loc) && !v.contains(&loc) {
            return Some(vec);
        }
    }

    None
}

fn follow_until(cur: (N, N), d: (N, N), s: &HashSet<(N, N)>, v: &mut HashSet<(N, N)>) -> ((N, N), N) {
    let mut end = cur;
    let mut next = (cur.0 + d.0, cur.1 + d.1);

    while s.contains(&next) {
        end = next;
        v.insert(end);
        next = (next.0 + d.0, next.1 + d.1)
    }

    let dist = match d {
        (0, _) => (end.1 - cur.1).abs(),
        (_, 0) => (end.0 - cur.0).abs(),
        _ => panic!("invalid dir"),
    };

    (end, dist)
}

fn get_turn(from: (N, N), to: (N,N)) -> char {
    match (from, to) {
        ((-1,0), (0,-1)) => 'L',
        ((0,-1), (1, 0)) => 'L',
        ((1, 0), (0, 1)) => 'L',
        ((0 ,1), (-1,0)) => 'L',
        ((-1 ,0), (0,1)) => 'R',
        ((1, 0), (0,-1)) => 'R',
        ((0, 1), (1, 0)) => 'R',
        ((0, -1), (-1, 0)) => 'R',
        _ => panic!("what! {:?} {:?}", from, to),
    }
}
*/

fn part_two(opcodes: &mut Vec<N>, start: (N,N), dir: (N,N), _scaffolds: HashSet<(N, N)>) -> N {
    let (mut _start, mut _dir) = (start, dir);
    /*
    let mut dist = 0;
    let mut prv = dir;
    let mut visited = HashSet::from([start]);

    loop {
        dir = get_next_position(start, &scaffolds, &visited).unwrap();
        //print!("{},", get_turn(prv, dir));
        (start, dist) = follow_until(start, dir, &scaffolds, &mut visited);
        prv = dir;
        //print!("{},", dist);
        break;
    }*/

    let a = [b'L',b',',b'1',b'2',b',',b'L',b',',b'8',b',',b'L',b',',b'8',10];
    let b = [b'L',b',',b'1',b'2',b',',b'R',b',',b'4',b',',b'L',b',',b'1',b'2',b',',b'R',b',',b'6',10];
    let c = [b'R',b',',b'4',b',',b'L',b',',b'1',b'2',b',',b'L',b',',b'1',b'2',b',',b'R',b',',b'6',10]; 
    let seq = [b'A',b',',b'B',b',',b'A',b',',b'C',b',',b'B',b',',b'A',b',',b'C',b',',b'A',b',',b'C',b',',b'B', 10,];
    let video = [b'n',b'\n', 10];

    let mut inputs = Vec::new();
    inputs.extend_from_slice(&seq);
    inputs.extend_from_slice(&a);
    inputs.extend_from_slice(&b);
    inputs.extend_from_slice(&c);
    inputs.extend_from_slice(&video);

    opcodes[0] = 2;
    let mut vm = VM::new(opcodes, 2);
    let mut part_two = 0;

    loop {
        if vm._get_next_opcode() == 3 { break; }
        vm.exec();
    }

    for i in 0..inputs.len() {
        loop {
            if vm._get_next_opcode() == 3 {
                vm._set_input(inputs[i] as N);
            }

            match vm.exec() {
                Status::Output(n) => part_two = n,
                Status::Input | Status::Exit => break,
                _ => (),
            };
        }
    }

    part_two
}

fn main() {
    let mut opcodes = read_input(17);
    let (p1, start, dir, scaffolds) = part_one(&opcodes);
    
    println!("Part one: {}", p1);
    println!("Part two: {}", part_two(&mut opcodes, start, dir, scaffolds));
}
