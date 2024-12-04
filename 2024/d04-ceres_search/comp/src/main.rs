use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/04").unwrap();

    let mut p1 = 0;
    let mut p2 = 0;

    let mut g = Vec::<Vec<char>>::new();

    for l in inp.lines().filter(|l| !l.is_empty()) {
        let c = l.chars().collect::<Vec<_>>();
        g.push(c);
    }

    for i in 0..g.len() {
        for j in 0..g.len() {
            if j + 2 < g[i].len()
                && i + 2 < g.len()
                && g[i][j] == 'M'
                && g[i][j + 2] == 'S'
                && g[i + 2][j] == 'M'
                && g[i + 1][j + 1] == 'A'
                && g[i + 2][j + 2] == 'S'
            {
                p2 += 1;
            }

            if j + 2 < g[i].len()
                && g[i][j] == 'S'
                && g[i][j + 2] == 'M'
                && i + 2 < g.len()
                && g[i + 2][j] == 'S'
                && g[i + 1][j + 1] == 'A'
                && g[i + 2][j + 2] == 'M'
            {
                p2 += 1;
            }

            if j + 2 < g[i].len()
                && g[i][j] == 'S'
                && g[i][j + 2] == 'S'
                && i + 2 < g.len()
                && g[i + 2][j] == 'M'
                && g[i + 1][j + 1] == 'A'
                && g[i + 2][j + 2] == 'M'
            {
                p2 += 1;
            }

            if j + 2 < g[i].len()
                && g[i][j] == 'M'
                && g[i][j + 2] == 'M'
                && i + 2 < g.len()
                && g[i + 2][j] == 'S'
                && g[i + 1][j + 1] == 'A'
                && g[i + 2][j + 2] == 'S'
            {
                p2 += 1;
            }

            if i + 3 < g.len()
                && g[i][j] == 'X'
                && g[i + 1][j] == 'M'
                && g[i + 2][j] == 'A'
                && g[i + 3][j] == 'S'
            {
                p1 += 1;
            }

            if i >= 3
                && g[i][j] == 'X'
                && g[i - 1][j] == 'M'
                && g[i - 2][j] == 'A'
                && g[i - 3][j] == 'S'
            {
                p1 += 1;
            }

            if j + 3 < g[i].len()
                && g[i][j] == 'X'
                && g[i][j + 1] == 'M'
                && g[i][j + 2] == 'A'
                && g[i][j + 3] == 'S'
            {
                p1 += 1;
            }

            if j >= 3
                && g[i][j] == 'X'
                && g[i][j - 1] == 'M'
                && g[i][j - 2] == 'A'
                && g[i][j - 3] == 'S'
            {
                p1 += 1;
            }

            if i + 3 < g.len()
                && j + 3 < g[i].len()
                && g[i][j] == 'X'
                && g[i + 1][j + 1] == 'M'
                && g[i + 2][j + 2] == 'A'
                && g[i + 3][j + 3] == 'S'
            {
                p1 += 1;
            }

            if i >= 3
                && j >= 3
                && g[i][j] == 'X'
                && g[i - 1][j - 1] == 'M'
                && g[i - 2][j - 2] == 'A'
                && g[i - 3][j - 3] == 'S'
            {
                p1 += 1;
            }

            if i + 3 < g.len()
                && j >= 3
                && g[i][j] == 'X'
                && g[i + 1][j - 1] == 'M'
                && g[i + 2][j - 2] == 'A'
                && g[i + 3][j - 3] == 'S'
            {
                p1 += 1;
            }

            if i >= 3
                && j + 3 < g[i].len()
                && g[i][j] == 'X'
                && g[i - 1][j + 1] == 'M'
                && g[i - 2][j + 2] == 'A'
                && g[i - 3][j + 3] == 'S'
            {
                p1 += 1;
            }
        }
    }

    println!("{g:?}");

    println!("{p1}");
    println!("{p2}");
}
