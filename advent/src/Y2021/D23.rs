use std::collections::{HashMap, VecDeque};

const LEN: usize = 11;
type Stack = VecDeque<([i32;LEN], Vec<Vec<i32>>, i32)>;

#[inline(always)]
fn part_one_is_done(c: &[Vec<i32>]) -> bool {
    c[0][0] == 1    && c[1][0] == 1 &&
    c[0][1] == 10   && c[1][1] == 10 &&
    c[0][2] == 100  && c[1][2] == 100 &&
    c[0][3] == 1000 && c[1][3] == 1000 
}

#[inline(always)]
fn part_two_is_done(c: &[Vec<i32>]) -> bool {
    c[0][0] == 1    && c[1][0] == 1    && c[2][0] == 1    && c[3][0] == 1 &&
    c[0][1] == 10   && c[1][1] == 10   && c[2][1] == 10   && c[3][1] == 10 &&
    c[0][2] == 100  && c[1][2] == 100  && c[2][2] == 100  && c[3][2] == 100 &&
    c[0][3] == 1000 && c[1][3] == 1000 && c[2][3] == 1000 && c[3][3] == 1000
}

fn get_result(is_part_two: bool, mut caves: Vec<Vec<i32>>, hallway: [i32; LEN]) -> i32 {
    if is_part_two {
        caves.insert(1, vec![1000, 10, 1, 100]);
        caves.insert(1, vec![1000, 100, 10, 1]);
    }

    let mut vis = HashMap::new();
    let mut stack = VecDeque::from_iter([(hallway, caves, 0)]);
    let mut ans = i32::MAX;

    while let Some((hallway, caves, cost)) = stack.pop_front() {
        if !is_part_two && part_one_is_done(&caves) || is_part_two && part_two_is_done(&caves) {
            ans = ans.min(cost);
            continue;
        }

        let key = (hallway, caves.clone());
        if let Some(&res) = vis.get(&key) {
            if res <= cost { continue; }
        }
        vis.insert(key, cost);

        for depth in 0..caves.len() {
            step_out_of_cave(depth, &caves, hallway, cost, &mut stack);
        }

        for (i, &pos) in hallway.iter().enumerate() {
            if pos == 0 { continue; }

            search_cave(
                i + 1..hallway.len(), 
                hallway, 
                cost, 
                &mut stack, 
                &caves, 
                |ptr: usize, pos: usize, d: i32| {(ptr - pos) as i32 + d },
                (i, pos)
            );

            search_cave(
                (1..i).rev(),
                hallway, 
                cost, 
                &mut stack, 
                &caves, 
                |ptr: usize, pos: usize, d: i32| {(pos - ptr) as i32 + d },
                (i, pos)
            );
        } 
    }

    ans
}

fn step_out_of_cave(depth: usize, caves: &Vec<Vec<i32>>, mut hallway: [i32; LEN], cost: i32, stack: &mut Stack) {
    'outer: for (i, &pos) in caves[depth].iter().enumerate() {
        if pos == 0 { continue; }
        for d in 0..depth {
            if caves[d][i] != 0 { continue 'outer; }
        }

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

fn search_cave<I, F>(
    direction: I, 
    mut hallway: [i32; LEN], 
    cost: i32,
    stack: &mut Stack,
    caves: &Vec<Vec<i32>>,
    steps: F,
    (pos, amph): (usize, i32), 
)
where 
    F: Fn(usize, usize, i32) -> i32,
    I: Iterator<Item = usize> 
{
    'outer: for ptr in direction {
        if hallway[ptr] != 0 { return; }
        if !matches!(ptr, 2 | 4 | 6 | 8) { continue; }

        let room = (ptr >> 1) - 1;
        let indx = (amph as f64).log10() as usize;

        if room != indx { continue; }

        for depth in 0..caves.len() {
            if !(caves[depth][room] == amph || caves[depth][room] == 0) {
                continue 'outer;
            }
        }

        hallway[pos] = 0;

        for depth in (0..caves.len()).rev() {
            if caves[depth][room] == 0 {
                let mut set  = caves.clone();
                set[depth][room] = amph;
                stack.push_back((hallway, set, cost + steps(ptr, pos, depth as i32 + 1) * amph));
                break;
            }
        }

        hallway[pos] = amph;
    }
}

pub fn solve() {
    let caves = std::fs::read_to_string("input/2021/23").unwrap().trim().lines()
        .skip(2)
        .take(2)
        .map(|l| {
            l
            .chars()
            .filter(|&c| c != ' ' && c != '#')
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

    println!("Part one: {part_one}", part_one = get_result(false, caves.clone(), [0; LEN]));
    println!("Part two: {part_two}", part_two = get_result(true, caves, [0; LEN]));
}
