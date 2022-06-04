fn main() {
    let content: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&content)
}

#[derive(Debug, Clone)]
struct Program {
    id: i32,
    connections: Vec<i32>,
}

fn solve_part_one(content: &String) {
    let mut programs: Vec<Program> = Vec::new();

    for line in content.trim().lines() {
        let line_parse: Vec<_> = line.split(" <->").collect();
        let connections: Vec<i32> = line_parse[1].trim().split(", ").map( | x | {
            x.parse::<i32>().unwrap()
        }).collect();

        let program: Program = Program { 
            id: line_parse[0].parse::<i32>().unwrap(), 
            connections: connections
        };

        programs.push(program);
    }

    let root: Program = programs[0].clone();
    let mut found: Vec<i32> = Vec::new();
    get_connections(root, &programs, &mut found);

    println!("Solution part 1: {}", found.len());
    solve_part_two(&mut programs);
}

fn solve_part_two(programs: &mut Vec<Program>) {
    let mut found: Vec<i32> = Vec::new();
    let mut groups: Vec<Vec<i32>> = Vec::new();
    let mut group_count: i32 = 1;
    let mut cur_len: usize = 0;

    for i in 0..programs.len() {
        let root: Program = programs[i].clone();
        get_connections(root, &programs, &mut found);

        if cur_len == 0 {
            cur_len = found.len();
        } else if found.len() > cur_len {
            group_count += 1;
            cur_len = found.len()
        }

        groups.push(found.clone());
    }

    println!("Solution part 2: {}", group_count);
}

fn get_connections(root: Program, programs: &Vec<Program>, found: &mut Vec<i32>) {
    if !found.contains(&root.id) {
        found.push(root.id);
    }

    for prog in root.connections {
        for i in 0..programs.len() {
            if programs[i].id == prog {
                if !found.contains(&prog) {
                    get_connections(programs[i].clone(), programs, found);
                }
            }
        }
    }
}