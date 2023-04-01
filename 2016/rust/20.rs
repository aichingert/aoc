// Advent of Code 2016, day 20
// (c) aichingert

type Range = (u64,u64);

fn add(ip: Range, firewall: &Vec<Range>) -> Vec<Range> {
    let mut out = Vec::new();
    let mut ip = ip;
    let mut pushed = false;

    for blocked in firewall {
        if blocked.1 + 1 < ip.0 {
            out.push(*blocked);
        } else if blocked.0 > ip.1 + 1 {
            if !pushed {
                out.push(ip);
                pushed = true;
            }
            out.push(*blocked);
        } else {
            ip.0 = ip.0.min(blocked.0);
            ip.1 = ip.1.max(blocked.1);
        }
    }

    if !pushed {
        out.push(ip);
    }

    out
}


fn main() {
    let mut firewall = Vec::new();
    std::fs::read_to_string("../input/20").unwrap()
        .lines()
        .for_each(|ip| { 
            let borders = ip.split_once('-').unwrap(); 
            firewall = add((borders.0.parse::<u64>().unwrap(),borders.1.parse::<u64>().unwrap()), &firewall);
        });

    println!("Part 1: {}", firewall[0].1+1);
    //println!("Part 2: {}", part2(&firewall));
}