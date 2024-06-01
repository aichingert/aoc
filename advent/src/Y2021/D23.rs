use std::collections::{HashMap, VecDeque};

const LEN: usize = 11;
type Stack = VecDeque<([i32;LEN], Vec<Vec<i32>>, i32)>;

fn is_done(c: &[Vec<i32>]) -> bool {
    c[0][0] == 1    && c[1][0] == 1 &&
    c[0][1] == 10   && c[1][1] == 10 &&
    c[0][2] == 100  && c[1][2] == 100 &&
    c[0][3] == 1000 && c[1][3] == 1000 
}

fn step_out_of_cave(depth: usize, caves: &Vec<Vec<i32>>, mut hallway: [i32; LEN], cost: i32, stack: &mut Stack) {
    for (i, &pos) in caves[depth].iter().enumerate() {
        if pos == 0 { continue; }
        if depth == 1 && caves[0][i] != 0 { continue; }

        let mut copy = caves.clone();
        copy[depth][i] = 0;

        // ...........
        // ##B#C#B#D##

        for j in (0..i * 2 + 1).step_by(2).rev() {
            if hallway[j] != 0 { break; }

            hallway[j] = pos;
            stack.push_back((hallway, copy.clone(), cost + ((((i * 2 + 2) - j) + depth) as i32) * pos));
            hallway[j] = 0;
        }
        for j in (i * 2 + 2..hallway.len()).step_by(2) {
            if hallway[j] != 0 { break; }

            hallway[j] = pos;
            stack.push_back((hallway, copy.clone(), cost + (((j - i * 2) + depth) as i32) * pos));
            hallway[j] = 0;
        }
    }
}

pub fn solve() {
    let inp = std::fs::read_to_string("input/2021/23").unwrap().trim().to_string();

    let hallway = [0; LEN];
    let configs = inp.lines()
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

    let mut vis = HashMap::new();
    let mut stack = VecDeque::from_iter([(hallway, configs, 0)]);
    let mut ans = i32::MAX;

    while let Some((hallway, configs, t)) = stack.pop_front() {
        if is_done(&configs) {
            ans = ans.min(t);
            continue;
        }

        let key = (hallway, configs.clone());
        if let Some(&res) = vis.get(&key) {
            if res <= t { continue; }
        }
        vis.insert(key, t);

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

        step_out_of_cave(0, &configs, hallway, t, &mut stack);
        step_out_of_cave(1, &configs, hallway, t, &mut stack);
    }

    println!("{ans}");
}
