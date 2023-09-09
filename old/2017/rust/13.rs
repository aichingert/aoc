// Advent of Code 2017, day 13
// (c) aichingert

fn part1(obsticles: &Vec<(i32,i32)>) -> i32 {
    let mut ans: i32 = 0;

    for i in 0..obsticles.len() {
        if obsticles[i].0 % ((obsticles[i].1 - 1) * 2)  == 0 {
            ans += obsticles[i].1 * obsticles[i].0;
        }
    }

    ans
}

fn part2(obsticles: &Vec<(i32,i32)>) -> i32 {
    let mut delay: i32 = 0;

    'outer: loop {
        for i in 0..obsticles.len() {
            if (obsticles[i].0 + delay) % ((obsticles[i].1 - 1) * 2) == 0 {
                delay += 1;
                continue 'outer;
            }
        }

        return delay;
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/13").unwrap()
        .lines()
        .map(|l| {
            let (pos,depth) = l.split_once(": ").unwrap();
            (pos.parse().unwrap(),depth.parse().unwrap())
        }).collect::<Vec<(i32,i32)>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
