// Advent of Code 2015, day 18
// (c) aichingert

fn neighbours(m: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut ans = 0;
    for y in -1..2 {
        for x in -1..2 {
            if y == 0 && x == 0 { continue; }
            if (y+(i as i32))>=0 && (x+(j as i32))>=0 && ((y+(i as i32)) as usize)<m.len() && ((x+(j as i32)) as usize)<m[i].len() && m[((y+(i as i32)) as usize)][((x+(j as i32)) as usize)] == '#' {
                ans += 1;
            }
        }
    }
    ans
}

fn corners(m: &mut Vec<Vec<char>>) {
    let l = m.len()-1;
    let lz = m[0].len()-1;
    m[0][0] = '#';
    m[0][lz] = '#';
    m[l][0] = '#';
    m[l][lz] = '#';
}

fn solve(inp: &Vec<Vec<char>>, part: bool) -> usize {
    let mut cur = inp.clone();
    if part { corners(&mut cur); }

    for _ in 0..100 {
        let mut n = cur.clone();
        for i in 0..inp.len() {
            for j in 0..inp[i].len() {
                let arnd = neighbours(&cur, i, j);

                n[i][j] = match cur[i][j] {
                    '#' => if arnd == 2 || arnd == 3 { '#' } else { '.' },
                    _ => if arnd == 3 { '#' } else { '.' },
                };
            }
        }
        cur = n;
        if part { corners(&mut cur); }
    }

    cur.iter().map(|l| l.iter().filter(|ch| *ch == &'#').count()).sum::<usize>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/18").unwrap();
    let inp = inp.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    println!("Part 1: {}", solve(&inp, false));
    println!("Part 2: {}", solve(&inp, true));
}
