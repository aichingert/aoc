use std::collections::{HashMap, HashSet}; 

type P = (usize, usize);

fn dfs(cur: P, end: P, d: u32, graph: &HashMap<P, Vec<(P, u32)>>, seen: &mut HashSet<(usize, usize)>) -> u32 {
    if cur == end {
        return d;
    }

    if !seen.insert(cur) {
        return 0;
    }

    let mut ans = 0;

    for &(vertex, dist) in graph.get(&cur).unwrap() {
        ans = ans.max(dfs(vertex, end, d + dist, graph, seen));
    }

    seen.remove(&cur);

    ans
}

fn follow_path(map: &Vec<Vec<char>>, d: u32, cur: P, me: P, vertices: &HashSet<P>, seen: &mut HashSet<P>, graph: &mut HashMap<P, Vec<(P, u32)>>) {
    let (y, x) = cur;

    if map[y][x] == '#' {
        return;
    }

    if !seen.insert(cur) {
        return;
    }

    if cur != me && vertices.contains(&cur) {
        graph.entry(me).and_modify(|l| l.push((cur, d))).or_insert(vec![(cur, d)]);
        //graph.entry(cur).and_modify(|l| l.push((me, d))).or_insert(vec![(me, d)]);
        return;
    }

    let d = d + 1;
    if y < map.len() - 1 {
        follow_path(map, d, (y + 1, x), me, vertices, seen, graph);
    }
    if y > 0 {
        follow_path(map, d, (y - 1, x), me, vertices, seen, graph);
    }

    follow_path(map, d, (y, x + 1), me, vertices, seen, graph);
    follow_path(map, d, (y, x - 1), me, vertices, seen, graph);
}

fn main() {
    let inp = std::fs::read_to_string("../input/23").unwrap().trim().to_string();
    let map = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut vertices = HashSet::new();

    for i in 1..map.len() - 1 {
        for j in 1..map[i].len() - 1 {
            if map[i][j] == '#' {
                continue;
            }

            let mut co = 0;

            for k in -1..2 {
                if k == 0 { continue; }

                let (r, c) = ((i as i32 + k) as usize, j);
                let (sr, sc) = (i, (j as i32 + k) as usize);

                if map[r][c] != '#' {
                    co += 1;
                }
                if map[sr][sc] != '#' {
                    co += 1;
                }
            }

            if co > 2 {
                vertices.insert((i, j));
            }
        }
    }

    let (cur, end) = ((0, 1), (map.len() - 1, map[0].len() - 2));

    vertices.insert(cur);
    vertices.insert(end);

    let mut graph = HashMap::new();

    for vertex in vertices.iter() {
        follow_path(&map, 0, *vertex, *vertex, &vertices, &mut HashSet::new(), &mut graph);
    }

    println!("{:?}", graph.len());
    for (v, l) in graph.iter() {
        println!("{:?}", v);

        for x in l {
            println!("  {:?}", x);
        }
    }


    println!("{}", dfs(cur, end, 0, &graph, &mut HashSet::new()));
       
}
