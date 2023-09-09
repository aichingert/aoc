// Advent of Code 2016, day 8
// (c) aichingert

fn solve(inp: &[&str]) {
    let mut map: [[char; 50];6] = [[' ';50];6];

    for i in 0..inp.len() {
        let vls = inp[i].split(' ').collect::<Vec<&str>>();

        match vls[0] {
            "rect" => {
                let (w,h) = vls[1].split_once('x').unwrap();
                let (w,h) = (w.parse::<usize>().unwrap(), h.parse::<usize>().unwrap());

                for y in 0..h {
                    for x in 0..w {
                        map[y][x] = '#';
                    }
                }
            },
            _ => {
                match vls[1] {
                        "row" => {
                        let (_, row) = vls[2].split_once('=').unwrap();
                        let row = row.parse::<usize>().unwrap();
                        let by = vls[4].parse::<usize>().unwrap();

                        for _ in 0..by%map[row].len() {
                            let mut next = map[row][0];
                            for j in 1..map[row].len() {
                                let swp = map[row][j];
                                map[row][j] = next;
                                next = swp;
                            }
                            map[row][0] = next;
                        }
                    },
                    _ => {
                        let (_, col) = vls[2].split_once('=').unwrap();
                        let col = col.parse::<usize>().unwrap();
                        let by = vls[4].parse::<usize>().unwrap();

                        for _ in 0..by%map.len() {
                            let mut next = map[0][col];
                            for j in 1..map.len() {
                                let swp = map[j][col];
                                map[j][col] = next;
                                next = swp;
                            }
                            map[0][col] = next;
                        }
                    },
                }
            }
        }
    }

    println!("Part 1: {}", map.iter().map(|row| row.iter().filter(|ch| **ch == '#').count()).sum::<usize>());
    println!("Part 2: ");
    for row in 0..map.len() {
        for col in 0..map[row].len() {
            print!("{}", map[row][col]);
        }
        println!();
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/08").unwrap();
    let inp = inp.trim().lines().collect::<Vec<&str>>();

    solve(&inp);
}
