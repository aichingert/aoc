fn part_one(xs: &Vec<u64>) -> u64 {
    let (mut o, mut t) = (0, 0);

    for i in 0..xs.len() - 1 {
        match xs[i + 1] - xs[i] {
            1 => o += 1,
            3 => t += 1,
            _ => (),
        };
    }

    o * t
}

fn part_two(xs: &Vec<u64>) -> u64 {
    let mut dp = vec![1];

    for i in 1..xs.len() {
        dp.push(0);

        for j in 1..=i {
            if xs[i] - xs[i - j] > 3 {
                break;
            }
            dp[i] += dp[i - j];
        }
    }

    *dp.last().unwrap()
}

fn main() {
    let xs = {
        let mut xs = std::fs::read_to_string("../input/10").unwrap().trim()
            .lines()
            .map(|l| l.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        xs.sort();
        xs.insert(0, 0);
        xs.push(xs.last().unwrap() + 3);
        xs
    };

    println!("part one: {}", part_one(&xs));
    println!("part two: {}", part_two(&xs));
}
