use std::collections::{HashMap,HashSet,VecDeque};

type Loc = (usize, usize);

struct Maze {
    keys:   HashMap<Loc, char>,
    doors:  HashMap<Loc, char>,
    walls:  HashSet<Loc>,
}

#[derive(Clone)]
struct State {
    inventory: HashSet<char>,
    visited: HashSet<Loc>,
    blocked: Vec<Loc>,
    starts: Vec<Loc>,
}

impl State {
    fn is_door_locked(&self, loc: Loc, maze: &Maze) -> bool {
        if let Some(door) = maze.doors.get(&loc) {
            let door_key = *door as u8 + 32;
            !self.inventory.contains(&(door_key as char))
        } else {
            false
        }
    }

    fn explore_path(&mut self, loc: Loc, maze: &Maze) {
        let mut bfs = VecDeque::from([loc]);

        while let Some((r, c)) = bfs.pop_front() {
            for (i, j) in [(0,1),(1,0),(0,-1),(-1,0)] {
                let (nr, nc) = (i + r as i32, j + c as i32);

                if nr < 0 || nc < 0 { continue; }

                let loc = (nr as usize, nc as usize);

                if self.visited.contains(&loc) || maze.walls.contains(&loc) { continue; }
                self.visited.insert(loc);

                //println!("{:?} - {}", loc, self.is_door_locked(loc, maze));
                if self.is_door_locked(loc, maze) {
                    self.blocked.push(loc);
                } else if maze.keys.contains_key(&loc) {
                    self.starts.push(loc);
                } else {
                    bfs.push_back(loc);
                }
            }
        }
    }
}

fn part_one(maze: &Maze, mut state: State, dst: u32) -> u32 { 
    if state.inventory.len() == maze.keys.len() {
        //println!("FIN: {:?}", state.inventory);
        return dst;
    }

    let mut dst = dst;
    let mut i = 0;

    while i < state.blocked.len() {
        let cur = maze.doors.get(&state.blocked[i]).unwrap();

        if state.inventory.contains(&cur) {
            let open = state.blocked.remove(i);
            state.starts.push(open);
        } else {
            i += 1;
        }
    }
    

    for i in 0..state.starts.len() {
        let start = state.starts.remove(i);
        let mut clone = state.clone();
        if let Some(key) = maze.keys.get(&start).and_then(|key| Some((*key as u8 - 32) as char)) {
            clone.inventory.insert(key);
        }

        clone.explore_path(start, maze);
        dst = dst.min(part_one(maze, clone, dst));

        state.starts.insert(i, start);
    }

    0
}

fn main() {
    let inp = std::fs::read_to_string("../input/18").unwrap().trim().to_string();
    let mut maze = Maze { keys: HashMap::new(), doors: HashMap::new(), walls: HashSet::new() };
    let mut start: Loc = (0,0);

    for (i, l) in inp.lines().enumerate() {
        for (j, c) in l.chars().enumerate() {
            match c {
                '#' => { maze.walls.insert((i, j)); },
                '@' => start = (i, j),
                'a'..='z' => { maze.keys.insert((i, j), c); },
                'A'..='Z' => { maze.doors.insert((i, j), c); },
                '.' => (),
                _ => panic!("invalid character {}", c),
            }
        }
    }

    let state = State { 
        inventory: HashSet::new(), 
        visited: HashSet::new(), 
        blocked: Vec::new(),
        starts: vec![start],
    };

    println!("Part one: {}", part_one(&maze, state, 0));
}
