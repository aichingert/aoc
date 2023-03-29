// Advent of Code 2016, day 16
// (c) aichingert

fn dragon_curve(a: &Vec<char>) -> Vec<char> {
    let mut b = a.clone();
    b.push('0');

    for i in 0..a.len() {
        match a[a.len()-i-1] {
            '1' => b.push('0'),
            '0' => b.push('1'),
            _ => panic!("invalid char")
        }
    }

    b
}

fn checksum(dragon: &Vec<char>) -> Vec<char> {
    if dragon.len() & 1 == 1 {
        return dragon.to_vec();
    }


    let mut sum = Vec::<char>::new();

    for i in (0..dragon.len()).step_by(2) {
        sum.push(if dragon[i] == dragon[i+1] {
            '1'
        } else {
            '0'
        });
    }

    checksum(&sum)
}

fn solve(dragon: &mut Vec<char>, disk_size: usize) -> String {
    while dragon.len() < disk_size {
        *dragon = dragon_curve(&dragon);
    }
    while dragon.len() > disk_size {
        dragon.pop();
    }

    checksum(&dragon).iter().collect::<String>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/16").unwrap().trim().chars().collect::<Vec<char>>();

    println!("Part 1: {}", solve(&mut inp.clone(), 272));
    println!("Part 2: {}", solve(&mut inp.clone(), 35651584));
}
