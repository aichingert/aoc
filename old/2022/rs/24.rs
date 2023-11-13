use std::collections::{HashMap, HashSet, VecDeque};

const VEC: [(i32,i32);5] = [(0,1),(1,0),(-1,0),(0,-1),(0,0)];

fn part_one(blzrd: &HashMap<char, Vec<(i32, i32)>>, len: (i32, i32)) -> i32 {
    let mut options = VecDeque::from([(1, -1, 0)]);
    let mut seen = HashSet::new();
    let blzrds = vec![('v', 1, 0), ('^', -1, 0), ('>', 0, 1), ('<', 0, -1)];
    let end = (len.0 - 1, len.1);

    while let Some((time, y, x)) = options.pop_front() {
        let time = time + 1;
        'outer: for (r, c) in VEC.iter() {
            let ny = r + y;
            let nx = c + x;

            if (ny, nx) == end {
                return time;
            }

            if nx < 0 || nx > len.1 || ny < 0 || ny > len.0 {
                continue;
            }

            for (dir, r, c) in blzrds.iter() {
                let np = ((ny - time * r).rem_euclid(len.0), (nx - time * c).rem_euclid(len.1));

                if blzrd[&dir].contains(&np) {
                    continue 'outer;
                }
            }

            if seen.insert((time, ny, nx)) {
                options.push_back((time, ny, nx));
            }
        }
    }

    -1
}

fn main() {
    let inp = std::fs::read_to_string("../input/24").unwrap().trim().to_string();
    let mut blzrd: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let (mut col, mut row) = (0, 0);

    for (i, e) in inp.lines().enumerate() {
        for (j, c) in e.chars().enumerate() {
            if !(c == '.' || c == '#') {
                let loc = (i as i32 - 1, j as i32 - 1);
                blzrd.entry(c).and_modify(|inner| inner.push(loc)).or_insert(vec![loc]);
            }

            col = j as i32 - 1;
        }
        row = i as i32 - 1;
    }

    println!("{row} - {col}");

    println!("Part one: {}", part_one(&blzrd, (row, col)));
}
