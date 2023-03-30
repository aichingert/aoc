// Advent of Code, day 17
// (c) aichingert

#[path="../../utils/rust/md5.rs"] mod md5;

fn moves(hash: &Vec<char>, loc: (i32,i32)) -> Vec<(i32,i32)> {
    let mut moves = Vec::new();
    
    if hash[0] >= 'b' && loc.0 > 0 {
        moves.push((-1,0));
    }
    if hash[1] >= 'b' && loc.0 + 1 < 4 {
        moves.push((1,0));
    }
    if hash[2] >= 'b' && loc.1 > 0 {
        moves.push((0,-1));
    }
    if hash[3] >= 'b' && loc.1 + 1 < 4 {
        moves.push((0,1));
    }

    moves
}

fn solve(start: String, loc: (i32,i32), paths: &mut Vec<String>) {
    if loc == (3,3) {
        paths.push(start);
        return;
    }

    let hash = md5::md5_utf8(&start).chars().collect::<Vec<char>>();

    for mv in moves(&hash, loc) {
        let dir = match mv {
            (-1,0) => "U",
            (1,0)  => "D",
            (0,-1) => "L",
            (0,1)  => "R",
            _ => panic!("Invalid direction")
        };

        solve(format!("{}{}",start,dir), (loc.0 + mv.0, loc.1 + mv.1), paths);
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/17").unwrap().trim().to_string();
    let mut paths = Vec::<String>::new();
    let mut part1: (usize, usize) = (0,usize::MAX);
    solve(inp.clone(), (0,0), &mut paths);

    for i in 0..paths.len() {
        if paths[i].len() < part1.1 {
            part1 = (i, paths[i].len());
        }
    }

    println!("Part 1: {}", &paths[part1.0][inp.len()..]);
}
