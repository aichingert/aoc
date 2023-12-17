use std::collections::{BinaryHeap, HashSet};
use std::cmp::{Ordering, Reverse};

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Point {
    y: usize,
    x: usize,
    s: u8,
    dir: u8,
    dis: u32,
}

impl Point {
    fn new(y: usize, x: usize, s: u8, dir: u8, dis: u32) -> Self {
        Self {
            y,
            x,
            s,
            dir,
            dis
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.dis.cmp(&other.dis))
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/17").unwrap().trim().to_string();
    let inp = inp.lines()
        .map(|l| l.chars().map(|ch| (ch as u8 - b'0') as u32).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (h, w) = (inp.len(), inp[0].len());

    let mut bfs = BinaryHeap::from([Reverse(Point::new(0, 0, 0, 0, 0))]);
    let mut seen = HashSet::new();

    while let Some(Reverse(Point { y, x, s, dir, dis })) = bfs.pop() {
        if !seen.insert((y, x, s, dir)) {
            continue;
        }

        let dst = dis + inp[y][x];

        if y == h - 1 && x == w - 1 && s >= 4 {
            println!("{}", dst - inp[0][0]);
            break;
        }

        let s = s + 1;

        if s >= 4 {
            match dir {
                0 | 1 => {
                    if y+1 < h { bfs.push(Reverse(Point::new(y + 1, x, 0, 3, dst))); }
                    if y > 0 { bfs.push(Reverse(Point::new(y - 1, x, 0, 2, dst))); }
                }
                2 | 3 => {
                    if x > 0 { bfs.push(Reverse(Point::new(y, x - 1, 0, 1, dst))); }
                    if x+1 < w { bfs.push(Reverse(Point::new(y, x + 1, 0, 0, dst))); }
                }
                _ => unreachable!(),
            }
        }

        if s < 10 {
            match dir {
                0 => if x+1 < w { bfs.push(Reverse(Point::new(y, x + 1, s, dir, dst))); }
                1 => if x > 0 { bfs.push(Reverse(Point::new(y, x - 1, s, dir, dst))); }
                2 => if y > 0 { bfs.push(Reverse(Point::new(y - 1, x, s, dir, dst))); }
                3 => if y+1 < h { bfs.push(Reverse(Point::new(y + 1, x, s, dir, dst))); }
                _ => unreachable!(),
            }
        }

        
    }


}
