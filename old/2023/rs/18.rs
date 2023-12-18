#![allow(non_snake_case)]

fn solve(inp: &Vec<&str>, parse: fn(&str) -> (i64, char)) -> i64 {
    let mut points = vec![(0, 0)];
    let mut b = 0;

    for line in inp {
        let pos = points[points.len() - 1];
        let (n, dir) = parse(line);
        b += n;

        points.push(match dir {
            'U'=> (pos.0 - n, pos.1),
            'D' => (pos.0 + n, pos.1),
            'R' => (pos.0, pos.1 + n),
            'L' => (pos.0, pos.1 - n),
            c => panic!("invalid {}", c),
        });
    }

    let A = (0..points.len() as i32)
        .map(|i| points[i as usize].1 
            * (points[(i - 1).rem_euclid(points.len() as i32) as usize].0
              -points[(i as usize + 1) % points.len()].0)
            )
        .sum::<i64>().abs() / 2; 
    let i = A - b / 2 + 1;

    i + b
}

fn main() {
    let inp = std::fs::read_to_string("../input/18").unwrap().trim().to_string();
    let inp = inp.split("\n").map(|s| s).collect::<Vec<_>>();
    
    let part_one = |s: &str| -> (i64, char) {
        let vals = s.split(' ').collect::<Vec<_>>();
        (vals[1].parse::<i64>().unwrap(), vals[0].chars().next().unwrap())
    };

    let part_two = |s: &str| -> (i64, char) {
        let vals = s.split(' ').collect::<Vec<_>>();
        let ch = vals[2].chars().collect::<Vec<_>>();
        let n = i64::from_str_radix(&ch[2..ch.len() - 2].iter().collect::<String>(), 16).unwrap();

        (n, match ch[ch.len() - 2] {
            '0' => 'R',
            '1' => 'D',
            '2' => 'L',
            '3' => 'U',
            c => panic!("invalid char {}", c),
        })
    };

    println!("Part one: {}", solve(&inp, part_one));
    println!("Part two: {}", solve(&inp, part_two));
} 
