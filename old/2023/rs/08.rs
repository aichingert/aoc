use std::collections::HashMap;

#[path = "../../utils/rust/math.rs"]
mod math;
use math::lcm;

fn solve(instr: &Vec<bool>, map: &HashMap<String, Vec<String>>, mut start: String, part_two: bool) -> usize {
    let (mut pos, mut ans) = (0, 1);

    loop {
        let cur = map.get(&start).unwrap();

        start = if instr[pos] {
            cur[1].clone()
        } else {
            cur[0].clone()
        };

        if !part_two && start.as_str() == "ZZZ" || part_two && start.ends_with("Z") {
            return ans;
        }

        (pos, ans) = ((pos + 1) % instr.len(), ans + 1);
    }
}

fn part_two(instr: &Vec<bool>, map: &HashMap<String, Vec<String>>, starts: Vec<String>) -> usize {
    starts.into_iter()
        .map(|start| solve(instr, map, start, true))
        .fold(1, |prv, nxt| lcm(prv, nxt))
}

fn main() {
    let inp = std::fs::read_to_string("../input/08").unwrap().trim().to_string();
    let (instr, network) = inp.split_once("\n\n").unwrap();
    let instr = instr.chars().map(|ch| ch == 'R').collect::<Vec<_>>();

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut starts = Vec::new();

    for line in network.trim().to_string().lines() {
        let (from, to) = line.split_once(" = (").unwrap();
        let (l, r) = to.split_once(", ").unwrap();

        if from.ends_with("A") {
            starts.push(from.to_string());
        }

        map.insert(from.to_string(), vec![l.to_string(), r[..r.len() - 1].to_string()]);
    }

    println!("Part one: {}", solve(&instr, &map, String::from("AAA"), false));
    println!("Part two: {}", part_two(&instr, &map, starts));
}
