#[path="intcode.rs"] mod intcode;
use intcode::{VM, Status, N, read_input};
use std::collections::{HashSet, VecDeque};

type Pos = (i32, i32);

#[derive(Debug, Copy, Clone)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn to_vec(&self) -> Pos {
        match self {
            Dir::North => (-1, 0),
            Dir::South => ( 1, 0),
            Dir::West  => (0, -1),
            Dir::East  => (0,  1),
        }
    }

    fn to_command(&self) -> N {
        match self {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }
}

struct Position {
    vm: VM,
    to: Dir,
    loc: Pos,
}

impl Position {
    fn new(vm: VM, to: Dir, loc: Pos) -> Self {
        Self {
            vm,
            to,
            loc
        }
    }

    fn next(&self) -> Vec<Self> {
        let loc = self.to.to_vec();
        let loc = (self.loc.0 + loc.0, self.loc.1 + loc.1);

        vec![Position::new(self.vm.clone(), Dir::North, loc),
            Position::new(self.vm.clone(), Dir::East, loc),
            Position::new(self.vm.clone(), Dir::West, loc),
            Position::new(self.vm.clone(), Dir::South, loc)]
    }
}

fn part_one(c: Pos, e: Pos, v: &mut HashSet<Pos>, d: u32, o: &HashSet<Pos>) -> Option<u32> {
    if c == e {
        return Some(d);
    } else if v.contains(&c) {
        return None;
    }
    v.insert(c);

    let routes = [(1,0),(-1,0),(0,1),(0,-1)].iter()
        .map(|(i, j)| (c.0 + *i, c.1 + *j))
        .filter(|(i,j)| {
            o.contains(&(*i, *j)) && !v.contains(&(*i, *j))
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for route in routes {
        if let Some(dst) = part_one(route, e, v, d + 1, o) {
            ans = ans.max(dst);
        }
    }

    Some(ans)
}

fn parse(opcodes: &Vec<N>) -> (HashSet<Pos>, Pos) {
    let vm = VM::new(&opcodes, 2);
    let mut bfs = VecDeque::from(vec![
        (Position::new(vm.clone(), Dir::North, (0,0))),
        (Position::new(vm.clone(), Dir::East, (0,0))),
        (Position::new(vm.clone(), Dir::South, (0,0))),
        (Position::new(vm.clone(), Dir::West, (0,0))),
    ]);

    let mut open = HashSet::from([(0, 0)]);
    let mut wall = HashSet::new();
    let mut oxygen_system = (0, 0);

    while let Some(mut position) = bfs.pop_front() {
        position.vm._set_input(position.to.to_command());
        let out;

        loop {
            match position.vm.exec() {
                Status::Output(n) => { out = n; break; },
                Status::Exit => panic!("what"),
                _ => (),
            }
        }

        match out {
            0 => {
                let loc = position.to.to_vec();
                wall.insert((position.loc.0 + loc.0, position.loc.1 + loc.1));
            }
            1 => {
                let mut first = true;
                for pos in position.next().into_iter() {
                    if first && (open.contains(&(pos.loc)) || wall.contains(&(pos.loc))) {
                        break;
                    }
                    first = false;

                    open.insert(pos.loc);
                    bfs.push_back(pos);
                }
            }
            2 => {
                let loc = position.to.to_vec();
                let pos = (position.loc.0 + loc.0, position.loc.1 + loc.1);
                open.insert(pos);
                oxygen_system = pos;
            }
            _ => panic!("invalid out"),
        };
    }

    (open, oxygen_system)
}

fn main() {
    let opcodes = read_input(15);
    let (open, oxygen_system) = parse(&opcodes);

    println!("Part one: {}", part_one((0,0), oxygen_system, &mut HashSet::new(), 0, &open).unwrap());
}
