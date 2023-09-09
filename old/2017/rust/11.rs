// Advent of Code 2017, day 11
// (c) aichingert

fn solve(dirs: &Vec<&str>) -> (u32,u32) {
    let mut furthest = 0;
    let mut loc = (0.0f32,0.0f32);

    for dir in dirs {
        match *dir {
            "n" => loc.0 += 1.,
            "nw" => { loc.0 += 0.5; loc.1 -= 0.5; },
            "ne" => { loc.0 += 0.5; loc.1 += 0.5; },
            "s" => loc.0 -= 1.,
            "sw" => { loc.0 -= 0.5; loc.1 -= 0.5; }
            "se" => { loc.0 -= 0.5; loc.1 += 0.5; }
            _ => {},
        };

        furthest = furthest.max((loc.0.abs() + loc.1.abs()) as u32);
    }

    ((loc.0.abs() + loc.1.abs()) as u32, furthest)
}

fn main() {
    let inp = std::fs::read_to_string("../input/11").unwrap();
    let inp = inp
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<&str>>();
    let (part1, part2) = solve(&inp);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
