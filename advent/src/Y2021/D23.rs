use std::collections::{HashMap, VecDeque};

// ans < 10528 

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
        let cur = i * 2 + 2;
        copy[depth][i] = 0;

        for j in (0..cur).rev() {
            if hallway[j] != 0 { break; }
            if matches!(j, 2 | 4 | 6 | 8) { continue; }

            hallway[j] = pos;
            stack.push_back((hallway, copy.clone(), cost + ((cur - j + 1 + depth) as i32) * pos));
            hallway[j] = 0;
        }

        for j in cur + 1..hallway.len() {
            if hallway[j] != 0 { break; }
            if matches!(j, 4 | 6 | 8) { continue; }

            hallway[j] = pos;
            stack.push_back((hallway, copy.clone(), cost + ((j - cur + 1 + depth) as i32) * pos));
            hallway[j] = 0;
        }
    }
}

fn search_cave_from_hallway(stack: &mut Stack) {
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

    while let Some((hallway, caves, cost)) = stack.pop_front() {
        if is_done(&caves) {
            ans = ans.min(cost);
            continue;
        }

        let key = (hallway, caves.clone());
        if let Some(&res) = vis.get(&key) {
            if res <= cost { continue; }
        }
        vis.insert(key, cost);

        step_out_of_cave(0, &caves, hallway, cost, &mut stack);
        step_out_of_cave(1, &caves, hallway, cost, &mut stack);

        let mut hllw = hallway;

        for (i, &pos) in hallway.iter().enumerate() {
            if pos == 0 { continue; }

            for r in i + 1..hallway.len() {
                if hallway[r] != 0 { break; }
                if !matches!(r, 2 | 4 | 6 | 8) { continue; }

                let room = (r >> 1) - 1;
                let amph = (pos as f64).log10() as usize;

                if room != amph || caves[0][room] != 0 { continue; }
                hllw[i] = 0;

                if caves[1][room] == pos {
                    let mut set  = caves.clone();
                    set[0][room] = pos;
                    stack.push_back((hllw, set, cost + ((r - i + 1) as i32) * pos));
                }

                if caves[1][room] == 0 {
                    let mut set  = caves.clone();
                    set[1][room] = pos;
                    stack.push_back((hllw, set, cost + ((r - i + 2) as i32) * pos));
                }

                hllw[i] = pos;
            }

            for l in (1..i).rev() {
                if hallway[l] != 0 { break; }
                if !matches!(l, 2 | 4 | 6 | 8) { continue };

                let room = (l >> 1) - 1;
                let amph = (pos as f64).log10() as usize;

                if room != amph || caves[0][room] != 0 { continue; }
                hllw[i] = 0;

                if caves[1][room] == pos {
                    let mut set  = caves.clone();
                    set[0][room] = pos;
                    stack.push_back((hllw, set, cost + ((i - l + 1) as i32) * pos));
                }

                if caves[1][room] == 0 {
                    let mut set  = caves.clone();
                    set[1][room] = pos;
                    stack.push_back((hllw, set, cost + ((i - l + 2) as i32) * pos));
                }

                hllw[i] = pos;
            }
        } 
    }

    println!("{ans}");
}
