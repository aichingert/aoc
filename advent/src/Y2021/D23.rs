use std::collections::{HashSet, VecDeque};

fn is_done(c: &[Vec<i32>]) -> bool {
    c[0][0] == 1    && c[1][0] == 1 &&
    c[0][1] == 10   && c[1][1] == 10 &&
    c[0][2] == 100  && c[1][2] == 100 &&
    c[0][3] == 1000 && c[1][3] == 1000 
}

pub fn solve() {
    let inp = std::fs::read_to_string("input/2021/23").unwrap().trim().to_string();

    let mut hallway = [0; 9];
    let mut configs = inp.lines()
        .skip(2)
        .take(2)
        .map(|l| {
            l
            .chars()
            .filter(|c| *c != ' ')
            .filter(|c| *c != '#')
            .map(|c| {
                match c {
                    'A' => 1,
                    'B' => 10,
                    'C' => 100,
                    'D' => 1000i32,
                    _ => panic!("{}", c),
                }
            })
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut vis = HashSet::new();
    let mut stack = VecDeque::from_iter([(hallway, configs, 0)]);
    stack.clear();
    hallway[2] = 10;
    hallway[4] = 100;
    stack.push_front((hallway, vec![vec![10,0,0,1000],vec![1,1000,100,1]], 0));
    let mut ans = i32::MAX;

    while let Some((hallway, configs, t)) = stack.pop_front() {
        if is_done(&configs) {
            println!("{t}");
            ans = ans.min(t);
            continue;
        }

        if !vis.insert((hallway, configs.clone())) {
            continue;
        }

        println!("{t}");
        print!("[");
        for i in 0..hallway.len() {
            match hallway[i] {
                1 => print!("A, "),
                10 => print!("B, "),
                100 => print!("C, "),
                1000 => print!("D, "),
                _ => print!("0, "),
            }
        }
        println!("]");
        print!("    ");
        for i in 0..configs[0].len() {
            match configs[0][i] {
                1 => print!("A"),
                10 => print!("B"),
                100 => print!("C"),
                1000 => print!("D"),
                _ => print!(" "),
            }
            match configs[1][i] {
                1 => print!("A    "),
                10 => print!("B    "),
                100 => print!("C    "),
                1000 => print!("D    "),
                _ => print!("    "),
            }
        }
        println!();
        println!();

        let mut hllw = hallway;

        for (i, &pos) in hallway.iter().enumerate() { 
            if pos == 0 { continue; }

            for r in i + 1..hallway.len() - 1 {
                if hallway[r] != 0 { break; }
                if (r + 1) & 1 != 0 { continue; }

                let room = ((r + 1) >> 1) - 1;
                if room != (pos as f64).log10() as usize { continue; }

                if configs[0][room] == 0 {
                    hllw[i] = 0;
                    if configs[1][room] == pos {
                        let mut copy = configs.clone();
                        copy[0][room] = pos;


                        stack.push_back((hllw, copy, t + ((r - i + 1) as i32) * pos));
                    }
                    if configs[1][room] == 0 {
                        let mut copy = configs.clone();
                        copy[1][room] = pos;
                        stack.push_back((hllw, copy, t + ((r - i + 2) as i32) * pos));
                    }
                    
                    hllw[i] = pos;
                }
            }

            for l in (1..i).rev() {
                if hallway[l] != 0 { break; }
                if (l + 1) & 1 != 0 { continue; }

                let room = ((l + 1) >> 1) - 1;
                if room != (pos as f64).log10() as usize { continue; }

                if configs[0][room] == 0 {
                    hllw[i] = 0;
                    if configs[1][room] == pos {
                        let mut copy = configs.clone();
                        copy[0][room] = pos;
                        stack.push_back((hllw, copy, t + ((1 + i - l) as i32) * pos));
                    }
                    if configs[1][room] == 0 {
                        let mut copy = configs.clone();
                        copy[1][room] = pos;

                        // 012345678
                        // ......C..
                        // #B#.#B#D#
                        // i - l + 1 => i: 6, l: 3 => 3 + 1 => 4

                        stack.push_back((hllw, copy, t + ((2 + i - l) as i32) * pos));
                    }
                    
                    hllw[i] = pos;
                }
            }
        }



        for (i, &pos) in configs[0].iter().enumerate() {
            if pos == 0 { continue; }

            let mut copy = configs.clone();
            copy[0][i] = 0;

            // 12521 => 12703
            //
            // A B  C   D
            // 1 10 100 1000

            for j in (0..i * 2 + 1).step_by(2).rev() {
                if hllw[j] != 0 { break; }

                hllw[j] = pos;

                // 012345678
                // ..B......
                // #B#C#.#D#
                //  0 1 2 3
                //
                // i: 2, j: 2
                //
                // 2 * 2 + 1 - 2 => 3
                //
                // i * 2 + 1 - j

                stack.push_back((hllw, copy.clone(), t + (((i * 2 + 2) - j) as i32) * pos));
                hllw[j] = 0;
            }

            for j in (i * 2 + 2..hallway.len()).step_by(2) {
                if hallway[j] != 0 { break; }

                hllw[j] = pos;

                // j - (i * 2 + 1)

                stack.push_back((hllw, copy.clone(), t + ((j - i * 2) as i32) * pos));
                hllw[j] = 0;
            }
        }

        for (i, &pos) in configs[1].iter().enumerate() {
            if pos == 0 { continue; }
            if configs[0][i] != 0 { continue; }

            let mut copy = configs.clone();
            copy[1][i] = 0;

            for j in (0..i * 2 + 1).step_by(2).rev() {
                if hllw[j] != 0 { break; }

                hllw[j] = pos;
                stack.push_back((hllw, copy.clone(), t + (((i * 2 + 1) - j) as i32 + 1) * pos));
                hllw[j] = 0;
            }
            for j in (i * 2 + 2..hallway.len()).step_by(2) {
                if hallway[j] != 0 { break; }

                hllw[j] = pos;
                stack.push_back((hllw, copy.clone(), t + ((j - i * 2 + 1) as i32 + 1) * pos));
                hllw[j] = 0;
            }
        }
    }

    println!("{ans}");
}
