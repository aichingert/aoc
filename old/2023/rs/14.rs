use std::collections::HashMap;

fn tilt(map: &mut Vec<Vec<char>>, dir: u8) {
    match dir {
        0 => {
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] == 'O' {
                        let mut d = i as i32;

                        while d - 1 >= 0 && map[(d - 1) as usize][j] == '.' {
                            map[(d - 1) as usize][j] = map[d as usize][j];
                            map[d as usize][j] = '.';
                            d -= 1;
                        }
                    }
                }
            }
        }
        1 => {
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[map.len() - i -1][j] == 'O' {
                        let mut y = map.len() - i - 1;

                        while y + 1 < map.len() && map[y + 1][j] == '.' {
                            map[y + 1][j] = map[y][j];
                            map[y][j] = '.';
                            y += 1;
                        }
                    }
                }
            }
        }
        2 => {
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][map[i].len() - j - 1] == 'O' {
                        let mut x = map[i].len() - j - 1;

                        while x + 1 < map[i].len() && map[i][x + 1] == '.' {
                            map[i][x + 1] = map[i][x];
                            map[i][x] = '.';
                            x += 1;
                        }
                    }
                }
            }

        }
        3 => {
            for i in 0..map.len() {
                for j in 0..map[i].len() {
                    if map[i][j] == 'O' {
                        let mut x = j as i32;

                        while x - 1 >= 0 && map[i][(x - 1) as usize] == '.' {
                            map[i][(x - 1) as usize] = map[i][x as usize];
                            map[i][x as usize] = '.';
                            x -= 1;
                        }
                    }
                }
            }
        }
        _ => panic!(),
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/14").unwrap().trim().to_string();
    let mut inp = inp.split("\n").map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut clone = inp.clone();
    
    let mut cycle = (0, 0);
    let mut cycler = HashMap::new();

    for i in 0..1000000000usize {
        tilt(&mut inp, 0);
        tilt(&mut inp, 3);
        tilt(&mut inp, 1);
        tilt(&mut inp, 2);

        if let Some(c) = cycler.get(&inp) {
            cycle = (i, *c);
            break;
        }

        cycler.insert(inp.clone(), i);
    }

    let x = (1000000000 - cycle.0) % (cycle.0 - cycle.1);
    println!("{}", x);

    for i in 0..cycle.0 {
        tilt(&mut clone, 0);
        tilt(&mut clone, 3);
        tilt(&mut clone, 1);
        tilt(&mut clone, 2);
    }

    for i in 0..x {
        tilt(&mut clone, 0);
        tilt(&mut clone, 3);
        tilt(&mut clone, 1);
        tilt(&mut clone, 2);
    }

    for i in 0..clone.len() {
        for j in 0..clone[i].len() {
            print!("{}", clone[i][j]);
        }
        println!();
    }

    println!("{:?}", cycle);

    let mut ans = 0;

    for i in 0..clone.len() {
        for j in 0..clone[i].len() {
            if clone[i][j] == 'O' {
                ans += clone.len() - i;
            }
        }
    }

    println!("{ans}");






}
