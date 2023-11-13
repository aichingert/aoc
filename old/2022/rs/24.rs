use std::collections::{HashMap, HashSet, VecDeque};

fn find_minimum_minutes_to_reach_end(blizzards: &HashMap<char, Vec<(i32, i32)>>, time: i32, start: (i32, i32), end: (i32, i32), rl: i32, cl: i32) -> i32 {
    let mut start = VecDeque::from([(time, (start))]);
    let mut seen = HashSet::new();

    while let Some((time, (r, c))) = start.pop_front() {
        let time = time + 1;

        'directions: for (y, x) in [(0,1),(1,0),(-1,0),(0,-1),(0,0)] {
            let adjacent = ((r + y), (c + x));

            if adjacent == end {
                return time;
            }

            if adjacent.0 < 0 || adjacent.1 < 0 || adjacent.0 > rl || adjacent.1 > cl {
                continue;
            }

            for (dir, y, x) in [('v', 1, 0), ('^', -1, 0), ('>', 0, 1), ('<', 0, -1)] {
                let next_r = (adjacent.0 - time * y).rem_euclid(rl);
                let next_c = (adjacent.1 - time * x).rem_euclid(cl);

                if blizzards[&dir].contains(&(next_r, next_c)) {
                    continue 'directions;
                }
            }

            if seen.insert((time, adjacent)) {
                start.push_back((time, adjacent));
            }
        }

        if start.len() == 0 {
            start.push_back((time, (r, c)));
        }
    }

    panic!("NO solution found!");
}

fn solve(blizzards: &HashMap<char, Vec<(i32, i32)>>, r: i32, c: i32) -> (i32, i32) {
    let start = (-1, 0);
    let end   = (r - 1, c);

    let part_one = find_minimum_minutes_to_reach_end(&blizzards, 0, start, end, r, c);
    let end_to_start = find_minimum_minutes_to_reach_end(&blizzards, part_one, end, start, r, c);

    (part_one, find_minimum_minutes_to_reach_end(&blizzards, end_to_start, start, end, r, c))
}

fn parse() -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {
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

    (blzrd, col, row)
}

fn main() {
    let (blizzards, col, row) = parse();
    let (part_one, part_two) = solve(&blizzards, row, col);

    println!("Part one: {:?}", part_one);
    println!("Part two: {:?}", part_two);
}
