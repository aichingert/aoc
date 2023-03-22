// Advent of Code 2019, day 10
// (c) aichingert

fn part1(inp: &Vec<Vec<char>>) -> usize {
    let mut cords = Vec::<(usize,usize)>::new();

    for (i,e) in inp.iter().enumerate() {
        for (j,ch) in e.iter().enumerate() {
            if ch == &'#' { cords.push((i,j)); }
        }
    }

    let mut w = 0;

    for i in 0..cords.len() {
        let mut points = Vec::new();

        for j in 0..cords.len() {
            if i == j { continue; }

            if !points.contains(&((cords[j].1 as f32 - cords[i].1 as f32).atan2(cords[j].0 as f32-cords[i].0 as f32))) {
                points.push((cords[j].1 as f32 - cords[i].1 as f32).atan2(cords[j].0 as f32-cords[i].0 as f32));
            }
        }

        if w < points.len() {
            w = points.len();
        }
    }

    w
}

fn part2(inp: &Vec<Vec<char>>) -> u32 {
    0
}

fn main() {
    let inp = std::fs::read_to_string("../input/10").unwrap().lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
