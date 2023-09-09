// Advent of Code 2017, day 3
// (c) aichingert

use std::collections::HashMap;

fn ulam_spiral_distance(n: i32) -> (i32, i32) {
    let k = (((n as f32).sqrt()-1.)/2.).ceil() as i32;
    let mut t = 2*k+1;
    let mut m = t*t;
    t -= 1;

    if n >= m-t { 
        return (k-(m-n),-k);
    } else { 
        m -= t;
    }

    if n >= m-t { 
        return (-k,-k+(m-n)); 
    } else { 
        m -= t; 
    }

    if n >= m-t { 
        (-k+(m-n),k)
    } else { 
        (k,k-(m-n-t))
    }
}

fn part1(n: i32) -> i32 {
    let (x,y) = ulam_spiral_distance(n);

    x.abs() + y.abs()
}

fn part2(n: i32) -> i32 {
    let mut spiral: HashMap<(i32,i32), i32> = HashMap::from([((0,0), 1)]);

    for idx in 2.. {
        let (x,y) = ulam_spiral_distance(idx);

        let mut acc = 0;

        for xx in -1..2 {
            for yy in -1..2 {
                if let Some(val) = spiral.get(&(x+xx,y+yy)) {
                    acc += val;
                }
            }
        }

        if acc > n {
            return acc;
        }

        spiral.insert((x,y), acc);
    }

    panic!("ahm...");
}

fn main() {
    let inp = std::fs::read_to_string("../input/03").unwrap().trim().parse::<i32>().unwrap();

    println!("Part 1: {}", part1(inp));
    println!("Part 2: {}", part2(inp));
}
