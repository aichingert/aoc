fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");


    solve_part_one(&input);
}

#[derive(Clone, Debug)]
struct Path {
    from: String,
    to: String,
}

fn solve_part_one(input: &String) {
    let mut paths: Vec<Path> = Vec::new();

    for line in input.lines() {
        let values: Vec<&str> = line.split('/').collect();
        paths.push(Path { from: values[0].to_string(), to: values[1].to_string() });
    }

    let roots: Vec<Path> = get_roots(&paths);
    let mut path: Vec<Vec<Path>> = Vec::new();
    let mut stored: Vec<Vec<Vec<Path>>> = Vec::new();

    for i in 0..roots.len() {
        let mut path_clones: Vec<Path> = paths.clone();
        find_connections(&mut path_clones, &mut path, &roots[i]);
        path.insert(0, vec![roots[i].clone()]);

        let mut offset: usize = 0; 
        for i in 0..path.len() {
            if path[i - offset].len() == 0 {
                path.remove(i - offset);
                offset += 1;
            }
        }
        stored.push(path.clone());
        path.clear();
    }

    let mut actual_paths: Vec<Vec<Path>> = Vec::new();
    let mut new_path: bool = true;

    'main: while new_path {
        for i in 0..roots.len() {
            let mut actual_path: Vec<Path> = Vec::new();
            set_together(&mut stored[i], 0, &mut actual_path, &roots[i]);

            if actual_path.len() == 0 {
                break 'main;
            }

            actual_paths.push(actual_path);
        }

        println!("{:?}", actual_paths);
    }
}

fn set_together(paths: &mut Vec<Vec<Path>>, idx: usize, path: &mut Vec<Path>, root: &Path) {
    println!("{:?}", paths[idx]);

    for i in 0..paths[idx].len() {
        if root.to == paths[idx][i].from || root.to == "1" && paths[idx][i].from == "9" {
            path.push(paths[idx][i].clone());
            set_together(paths, idx + 1, path, &paths[idx][i].clone());
        }
    }
}

fn get_roots(paths: &Vec<Path>) -> Vec<Path> {
    let mut roots: Vec<Path> = Vec::new();

    for i in 0..paths.len() {
        if paths[i].from == "0" {
            roots.push(paths[i].clone());
        }
    }

    roots
}

fn find_connections(paths: &mut Vec<Path>, path: &mut Vec<Vec<Path>>, root: &Path) {
    let mut connections: Vec<Path> = Vec::new();
    let mut offset: usize = 0;

    for i in 0..paths.len() {
        if root.to == paths[i - offset].from || root.to == "1" && paths[i - offset].from == "9" {
            connections.push(paths[i - offset].clone());
            paths.remove(i - offset);
            offset += 1;
        }
    }

    let len = connections.len();
    path.push(connections.clone());

    for i in 0..len {
        find_connections(paths, path, &connections[i]);
    }
}