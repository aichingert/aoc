use std::collections::{HashMap, HashSet, VecDeque};

type N = i128;

fn dfs(ms: N, y: N, x: N, s: N, map: &Vec<Vec<char>>, filter: &mut HashSet<(N, N)>) -> N {
    if !filter.insert((y, x, s)) {
        return 0;
    } 

    let (ay, ax) = (y.rem_euclid(map.len() as N), x.rem_euclid(map[0].len() as N));
    let (ay, ax) = (ay as usize, ax as usize);

    if map[ay][ax] == '#' {
        return 0;
    }

    if s >= ms {
        return 1;
    }

    let mut res: N = 0;
    let s = s + 1;

    res += dfs(ms, y + 1, x, s, map, filter);
    res += dfs(ms, y - 1, x, s, map, filter);
    res += dfs(ms, y, x + 1, s, map, filter);
    res += dfs(ms, y, x - 1, s, map, filter);

    res
}


fn main() {
    let inp = std::fs::read_to_string("../input/21").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut start = (0, 0);

    'f: for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if inp[i][j] == 'S' {
                start = (i as N, j as N);
                break 'f;
            }
        }
    }

    println!("{}", dfs(5000, start.0, start.1, 0, &inp, &mut HashSet::new()));

    /*
    let mut bfs = VecDeque::from([(start, 0)]);
    let mut sen = HashSet::new();
    let mut ans = 0;

    while let Some(((y, x), s)) = bfs.pop_front() {  
        if !sen.insert((y, x, s)) {
            continue;
        }

        let s = s + 1;

        if s > STEPS {
            ans += 1;
            continue;
        }

        for (r, c) in [(0,1),(1,0),(0,-1),(-1,0)] {
            let (ny, nx) = (y + r, x + c);
            let (ay, ax) = (ny.rem_euclid(inp.len() as i64), nx.rem_euclid(inp[0].len() as i64));
            let (ay, ax) = (ay as usize, ax as usize);

            if inp[ay][ax] != '#' {
                bfs.push_back(((ny, nx), s));
            }
        }
    }

    println!("{:?}", ans);
    */
}
