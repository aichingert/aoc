// Advent of Code 2017, day 19
// (c) aichingert

fn solve(start: (usize,usize), map: &Vec<Vec<char>>) -> (String, usize) {
    let mut ans = String::new();
    let mut dir = (1,0);
    let mut cur = start;
    let mut steps = 0;

    loop {
        if map[cur.0][cur.1] == ' ' {
            return (ans, steps);
        }

        let prev = map[cur.0][cur.1];
        cur = ((cur.0 as i32 + dir.0) as usize,(cur.1 as i32 + dir.1) as usize);
        steps += 1;

        if map[cur.0][cur.1].is_alphanumeric() {
            ans.push(map[cur.0][cur.1]);
        } else if map[cur.0][cur.1] == '+' {
            for vec in [(1,0),(0,1),(-1,0),(0,-1)] {
                let y = (cur.0 as i32 + vec.0) as usize;
                let x = (cur.1 as i32 + vec.1) as usize;

                if map[y][x] != ' ' && map[y][x] != prev {
                    dir = vec;
                }
            }
        }
    }
}

fn parse() -> ((usize,usize), Vec<Vec<char>>) {
    let mut map = std::fs::read_to_string("../input/19").unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    map.push(vec![' ';map[0].len()]);
    let start = (0..map[0].len()).find(|&i| map[0][i] == '|').unwrap();
    ((0,start),map)
}

fn main() {
    let (start, map) = parse();
    let (part1,part2) = solve(start, &map);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
