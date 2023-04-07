// Advent of Code 2016, day 22
// (c) aichingert

#[derive(Debug)]
struct Node {
    y: u32,
    size: u32,
    used: u32,
}

impl Node {
    fn new(y: u32, size: u32, used: u32) -> Self {
        Self { y, size, used }
    }
}

fn parse() -> Vec<Node> {
    let mut nodes = Vec::<Node>::new();

    for line in std::fs::read_to_string("../input/22").unwrap().lines().skip(2) {
        let vls = line.split(' ').filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let loc = vls[0].split('-').collect::<Vec<&str>>();

        nodes.push(Node::new(
                loc[2][1..].parse().unwrap(), 
                vls[1][..vls[1].len()-1].parse().unwrap(),
                vls[2][..vls[2].len()-1].parse().unwrap()));
    }

    nodes
}

fn part1(nodes: &Vec<Node>) -> usize {
    let mut sum = 0;

    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i == j { continue; }
            if nodes[i].used > 0 && nodes[i].used <= nodes[j].size - nodes[j].used {
                sum += 1;
            }
        }
    }

    sum
}

fn part2(nodes: &Vec<Node>) {
    for i in 0..nodes.len() {
        if nodes[i].y == 0 { println!(); }
        if nodes[i].used >= 490 { print!("#"); }
        else if nodes[i].used == 0 { print!("_"); }
        else { print!("."); }
    }
}

fn main() {
    let nodes = parse();

    println!("Part 1: {}", part1(&nodes));
    part2(&nodes);
}
