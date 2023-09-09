// Advent of Code 2015, day 6
// (c) aichingert

fn solve(inp: &str, part: bool) -> u32 {
    let mut m: [[i8; 1000]; 1000] = [[0; 1000]; 1000];

    for l in inp.lines() {
        let (vls, fx, fy, tx, ty) = parse(l);

        for y in fy..=ty {
            for x in fx..=tx {
                if vls == "on" {
                    if part {
                        m[y][x] = 1
                    } else {
                        m[y][x] += 1
                    }
                } else if vls == "off" {
                    if part {
                        m[y][x] = 0
                    } else {
                        m[y][x] = std::cmp::max(0, m[y][x] - 1)
                    }
                } else {
                    if part {
                        m[y][x] = 1 - m[y][x]
                    } else {
                        m[y][x] += 2
                    }
                }
            }
        }
    }

    m.iter()
        .map(|l| l.iter().map(|val| *val as u32).sum::<u32>())
        .sum::<u32>()
}

fn parse(line: &str) -> (&str, usize, usize, usize, usize) {
    let mut vls: Vec<&str> = line.split(' ').collect();
    if vls.len() == 4 {
        vls.insert(0, "work");
    }
    let (fx, fy): (&str, &str) = vls[2].split_once(',').unwrap();
    let (fx, fy): (usize, usize) = (fx.parse().unwrap(), fy.parse().unwrap());
    let (tx, ty): (&str, &str) = vls[4].split_once(',').unwrap();
    let (tx, ty): (usize, usize) = (tx.parse().unwrap(), ty.parse().unwrap());
    (vls[1], fx, fy, tx, ty)
}

fn main() {
    let inp = std::fs::read_to_string("../input/06").unwrap();

    println!("Part 1: {}", solve(&inp, true));
    println!("Part 2: {}", solve(&inp, false));
}
