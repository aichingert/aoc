use std::collections::{HashMap, HashSet};

type V2 = (u64, u64);

fn add(a: V2, b: V2) -> (u64, u64) {
    (a.0 + b.0, a.1 + b.1)
}

fn p(scale: u64, coord: V2) -> (u64, u64) {
    (coord.0 * scale, coord.1 * scale)
}

fn find_x(prize_x: u64, ax: u64, bx: u64) -> u64 {
    let mut x = prize_x / bx - 1;
    println!("{prize_x} | {x} {}", bx * x);

    for i in 0..x {
        let sax = i * ax;
        let sbx = (x - i) * bx;

        //println!("{prize_x} | {}", sax + sbx);
        if prize_x == sax + sbx {
            return i;
        }
    }

    u64::MAX
}

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/13").unwrap();

    let entries = inp.split("\n\n");
    let mut p1 = 0;
    let mut p2 = 0;

    for (i, entry) in entries.filter(|l| !l.trim().is_empty()).enumerate() {
        let l = entry.lines().collect::<Vec<_>>();
        let a = l[0].split(' ').collect::<Vec<_>>();
        let a = (a[2][2..a[2].len() - 1].parse::<u64>().unwrap(), a[3][2..].parse::<u64>().unwrap());
        let b = l[1].split(' ').collect::<Vec<_>>();
        let b = (b[2][2..b[2].len() - 1].parse::<u64>().unwrap(), b[3][2..].parse::<u64>().unwrap());

        let xy = l[2].split(' ').collect::<Vec<_>>();
        let prize = (xy[1][2..xy[1].len() - 1].parse::<u64>().unwrap(), xy[2][2..].parse::<u64>().unwrap());
        let mut ans = u64::MAX;
        
        for i in 0..100 {
            for j in 0..100 {
                if add(p(j, a), p(i, b)) == prize {
                    ans = ans.min(j * 3 + i);
                }
            }
        }

        if ans != u64::MAX {
            p1 += ans;
        }

        ans = u64::MAX;
        let prize = (prize.0 + 10000000000000, prize.1 + 10000000000000); 

        // println!("{} {} {}", prize.0 % a.0, prize.0 % b.0, prize.0 % (a.0 + b.0));

        // a[0] * n + b[0] * x = p[0]
        //
        // a[1] * n + b[1] * x = p[1]

        for i in 0..{
            for j in 0.. {
                let v = add(p(j, a), p(i, b));
                if v.0 > prize.0 || v.1 > prize.1 {
                    break;
                }

                if v == prize {
                    ans = ans.min(j * 3 + i);
                }
            }

            println!("{ans:?}");
        }

        if ans != u64::MAX {
            p2 += ans;
        }
    }

    println!("{p1}");
    println!("{p2}");
}
