// Advent of Code 2015, day 6
// (c) aichingert

fn part1(inp: &str) -> u32 {
    let mut m: [[bool;1000];1000] = [[false;1000];1000];

    for l in inp.lines() {
        let (vls,fx,fy,tx,ty) = parse(l);
        for y in fy..=ty {
            for x in fx..=tx {
                if vls[0] != "turn" {
                    m[y][x] = !m[y][x];
                } else {
                    if vls[1] == "on" {
                        m[y][x] = true;
                    } else{
                        m[y][x] = false;
                    }
                }
            }
        }
    }

    m.iter().map(|l| l.iter().map(|val| if *val { 1 } else { 0 }).sum::<u32>()).sum::<u32>()
}

fn part2(inp: &str) -> u32 {
    let mut m: [[i8;1000];1000] = [[0;1000];1000];

    for l in inp.lines() {
        let (vls,fx,fy,tx,ty) = parse(l);
        for y in fy..=ty {
            for x in fx..=tx {
                if vls[0] != "turn" {
                    m[y][x] += 2;
                } else {
                    if vls[1] == "on" {
                        m[y][x] += 1;
                    } else{
                        m[y][x] = std::cmp::max(0, m[y][x]-1);
                    }
                }
            }
        }
    }

    m.iter().map(|l| l.iter().map(|val| *val as u32).sum::<u32>()).sum::<u32>()
}

fn parse(line: &str) -> (Vec<&str>,usize,usize,usize,usize) {
    let mut vls: Vec<&str> = line.split(' ').collect();
    if vls.len() == 4 { vls.insert(0,"work"); }
    let (fx,fy): (&str,&str) = vls[2].split_once(',').unwrap();
    let (fx,fy): (usize, usize) = (fx.parse().unwrap(), fy.parse().unwrap());
    let (tx,ty): (&str,&str) = vls[4].split_once(',').unwrap();
    let (tx,ty): (usize, usize) = (tx.parse().unwrap(), ty.parse().unwrap());
    (vls,fx,fy,tx,ty)
}
 
fn main() {
    let inp = std::fs::read_to_string("../input/06").unwrap();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
