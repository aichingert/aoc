// Advent of Code 2016, day 15
// (c) aichingert

fn solve(disks: &Vec<(u32,u32)>) -> u32 {
    let mut time: u32 = 0;

    'sim: loop {
        for i in 0..disks.len() {
            if (disks[i].1 + time + (i as u32) + 1) % disks[i].0 != 0 {
                time += 1;
                continue 'sim;
            }
        }
        
        return time;
    }
}

fn parse() -> Vec<(u32, u32)> {
    let inp = std::fs::read_to_string("../input/15").unwrap();
    let mut disks = Vec::<(u32,u32)>::new();
    
    for line in inp.lines() {
        let vls = line.split(' ').collect::<Vec<&str>>();
        disks.push((vls[3].parse::<u32>().unwrap(), vls[vls.len()-1][..vls[vls.len()-1].len()-1].parse::<u32>().unwrap()));
    }

    disks
}

fn main() {
    let mut disks = parse();

    println!("Part 1: {}", solve(&disks));
    disks.push((11,0));
    println!("Part 2: {}", solve(&disks));
}
