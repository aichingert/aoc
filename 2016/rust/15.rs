// Advent of Code 2016, day 15
// (c) aichingert

fn part1(disks: &Vec<(u32,u32)>) -> u32 {
    let mut time: u32 = 0;

    'sim: loop {
        let mut simulation = time;

        for i in 0..disks.len() {
            simulation += 1;

            if (disks[i].1 + simulation) % disks[i].0 != 0 {
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
        disks.push((vls[3].parse::<u32>().unwrap(), 
                    vls[vls.len()-1][..vls[vls.len()-1].len()-1].parse::<u32>().unwrap()));
    }

    disks
}

fn main() {
    let disks = parse();

    println!("Part 1: {}", part1(&disks));
}
