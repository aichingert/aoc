// Advent of Code 2015, day 14
// (c) aichingert

fn part1(reindeers: &[Reindeer]) -> u32 {
    reindeers.iter().map(|r| r.position(2503)).max().unwrap()
}

fn part2(reindeers: &[Reindeer]) -> u32 {
    let mut points = vec![0u32; reindeers.len()];

    for time in 1..=2503 {
        let mut fur = 0usize;
        let mut max = 0u32;
     
        for (i,r) in reindeers.iter().enumerate() {
            let dis = r.position(time);
            if dis >= max {
                max = dis;
                fur = i;
            }
        }

        points[fur] += 1;
    }

    *points.iter().map(|p| p).max().unwrap()
}

struct Reindeer {
    speed: u32,
    duration: u32,
    rest: u32
}

impl Reindeer {
    fn position(&self, time: u32) -> u32 {
        let t = time / (self.duration + self.rest);
        let r = time % (self.duration + self.rest);

        t * self.speed * self.duration + self.duration.min(r) * self.speed
    }
}

fn main() {
    let reindeers = std::fs::read_to_string("../input/14").unwrap().lines().map(|l| {
        let vls = l.split(' ').collect::<Vec<&str>>();
        Reindeer { speed: vls[3].parse().unwrap(), duration: vls[6].parse().unwrap(), rest: vls[13].parse().unwrap() }
    }).collect::<Vec<Reindeer>>();

    println!("Part 1: {}", part1(&reindeers));
    println!("Part 2: {}", part2(&reindeers));
}
