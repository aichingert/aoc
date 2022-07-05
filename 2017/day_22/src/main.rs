fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

#[derive(Clone, Debug)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged
}

fn solve_part_one(input: &String) {
    let mut map: Vec<Vec<NodeState>> = Vec::new();
    let mut pos: (usize, usize) = (0, 0);
    let mut pointing: i32 = 90; // Degree
    let mut bursts: i32 = 0;

    for line in input.lines() {
        let mut characters: Vec<NodeState> = Vec::new();

        for c in line.chars() {
            if c == '.' {
                characters.push(NodeState::Clean)
            } else {
                characters.push(NodeState::Infected);
            }
        }

        map.push(characters);
    }

    pos.0 = map.len() / 2;
    pos.1 = map[pos.0].len() / 2;

    for _ in 0..10000 {
        match map[pos.0][pos.1] {
            NodeState::Clean => {
                map[pos.0][pos.1] = NodeState::Infected;
                pointing += 90;
                bursts += 1;
            },
            NodeState::Infected => {
                map[pos.0][pos.1] = NodeState::Clean;
                pointing -= 90;
            }
            _ => {}
        }

        if pointing < 0 {
            pointing = 270;
        }

        pointing %= 360;

        match pointing {
            0 => {
                if pos.1 + 1 >= map[pos.0].len() {
                    expand(2, &mut map);
                }

                pos.1 += 1;
            },
            90 => {
                if pos.0 as i32 - 1 < 0 {
                    expand(1, &mut map);
                } else {
                    pos.0 -= 1;
                }
            },
            180 => {
                if pos.1 as i32 - 1 < 0 {
                    expand(3, &mut map);
                } else {
                    pos.1 -= 1;
                }
            },
            270 => {
                if pos.0 + 1 >= map.len() {
                    expand(0, &mut map);
                }

                pos.0 += 1;
            },
            _ => {}
        }
    }

    println!("Solution part one: {}", bursts);
}

fn solve_part_two(input: &String) {
    let mut map: Vec<Vec<NodeState>> = Vec::new();
    let mut pos: (usize, usize) = (0, 0);
    let mut pointing: i32 = 90; // Degree
    let mut bursts: i32 = 0;

    for line in input.lines() {
        let mut characters: Vec<NodeState> = Vec::new();

        for c in line.chars() {
            if c == '.' {
                characters.push(NodeState::Clean)
            } else {
                characters.push(NodeState::Infected);
            }
        }

        map.push(characters);
    }

    pos.0 = map.len() / 2;
    pos.1 = map[pos.0].len() / 2;

    for _ in 0..10000000 {
        match map[pos.0][pos.1] {
            NodeState::Clean => {
                map[pos.0][pos.1] = NodeState::Weakened;
                pointing += 90;
            },
            NodeState::Weakened => {
                map[pos.0][pos.1] = NodeState::Infected;
                bursts += 1;
            },
            NodeState::Infected => {
                map[pos.0][pos.1] = NodeState::Flagged;
                pointing -= 90;
            },
            NodeState::Flagged => {
                map[pos.0][pos.1] = NodeState::Clean;
                pointing += 180;
            }
        }

        if pointing < 0 {
            pointing = 270;
        }

        pointing %= 360;

        match pointing {
            0 => {
                if pos.1 + 1 >= map[pos.0].len() {
                    expand(2, &mut map);
                }

                pos.1 += 1;
            },
            90 => {
                if pos.0 as i32 - 1 < 0 {
                    expand(1, &mut map);
                } else {
                    pos.0 -= 1;
                }
            },
            180 => {
                if pos.1 as i32 - 1 < 0 {
                    expand(3, &mut map);
                } else {
                    pos.1 -= 1;
                }
            },
            270 => {
                if pos.0 + 1 >= map.len() {
                    expand(0, &mut map);
                }

                pos.0 += 1;
            },
            _ => {}
        }
    }

    println!("Solution part two: {}", bursts);
}

fn expand(dir: u8, map: &mut Vec<Vec<NodeState>>) {
    match dir {
        0 => { // expand below
            map.insert(map.len(), vec![NodeState::Clean; map[0].len()]);
        },
        1 => { // expand up
            map.insert(0, vec![NodeState::Clean; map[0].len()]);
        },
        2 => { // expand right
            for i in 0..map.len() {
                map[i].push(NodeState::Clean);
            }
        },
        3 => { // expand left
            for i in 0..map.len() {
                map[i].insert(0, NodeState::Clean);
            }
        },
        _ => {}
    }
}