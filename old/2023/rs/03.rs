fn main() {
    let inp = std::fs::read_to_string("../input/03").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut ans = 0;

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if !(inp[i][j] >= '0' && inp[i][j] <= '9') && inp[i][j] == '*' {
                let mut seen = Vec::new();
                let mut has = Vec::new();
                for y in [(-1,0),(0,-1),(1,0),(0,1),(1,1),(1,-1),(-1,-1),(-1,1)] {
                    let (nx, ny) = (j as i32 + y.0, i as i32 + y.1);

                    if  nx < 0 || ny < 0 || nx >= inp[i].len() as i32 || ny >= inp.len() as i32 {
                        continue;
                    }

                    let (ny, nx) = (ny as usize, nx as usize);

                    if inp[ny][nx] >= '0' && inp[ny][nx] <= '9' {
                        println!("{ny}:{nx} => {}", inp[ny][nx]);
                        let mut chs = Vec::new();
                        let mut x = 1 + nx;

                        if seen.contains(&(ny, nx)) {
                            continue;
                        }

                        while x < inp[ny].len() && inp[ny][x] >= '0' && inp[ny][x] <= '9' {
                            seen.push((ny, x));
                            chs.push(inp[ny][x]);
                            x += 1;
                        }

                        x = nx;
                        while x > 0 && inp[ny][x] >= '0' && inp[ny][x] <= '9' {
                            seen.push((ny, x));
                            chs.insert(0, inp[ny][x]);
                            x -= 1;
                        }

                        if inp[ny][x] >= '0' && inp[ny][x] <= '9' {
                            seen.push((ny, x));
                            chs.insert(0, inp[ny][x]);
                        }

                        let s = chs.iter().collect::<String>();
                        //ans += s.parse::<u32>().unwrap();
                        has.push(s.parse::<u32>().unwrap());
                    }
                }

                if has.len() == 2 {
                    ans += has[0] * has[1];
                }
                println!("{:?}", seen);
            }
        }
    }

    println!("{}", ans);

}
