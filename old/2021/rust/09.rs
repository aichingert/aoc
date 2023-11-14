use std::collections::{HashSet, VecDeque, BinaryHeap};

fn get_low_points(inp: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            let mut is_low_point: bool = true;

            for (r,c) in [(0,1),(1,0),(0,-1),(-1,0)] {
                let (y, x) = (i as i32 + r, j as i32 + c);

                if y < 0 || y >= inp.len() as i32 || x < 0 || x >= inp[i].len() as i32 {
                    continue;
                }

                if inp[y as usize][x as usize] <= inp[i][j] {
                    is_low_point = false;
                    break;
                }
            }

            if is_low_point {
                low_points.push((i, j));
            }
        }
    }

    low_points
}

fn part_one(inp: &Vec<Vec<u8>>, low_points: &Vec<(usize, usize)>) -> u32 {
    low_points.iter().map(|(i, j)| (inp[*i][*j] + 1) as u32).sum::<u32>()
}

fn part_two(inp: &Vec<Vec<u8>>, low_points: &Vec<(usize, usize)>) -> usize {
    let mut sizes = BinaryHeap::new();

    for i in 0..low_points.len() {
        let mut seen = HashSet::from([low_points[i]]);
        let mut start = VecDeque::from([low_points[i]]);

        while let Some((r, c)) = start.pop_front() {
            for (y, x) in [(0, 1), (1,0), (0,-1), (-1,0)] {
                let (nr, nc) = (r as i32 + y, c as i32 + x);

                if nr < 0 || nc < 0 || nr >= inp.len() as i32 || nc >= inp[0].len() as i32 {
                    continue;
                }

                let (nr, nc) = (nr as usize, nc as usize);

                if inp[nr][nc] != 9 && inp[nr][nc] >= inp[r][c] + 1 && seen.insert((nr, nc)) {
                    start.push_back((nr, nc));
                }
            }
        }

        sizes.push(seen.len());
    }

    sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap()
}

fn main() {
    let inp = std::fs::read_to_string("../input/09").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l.chars().map(|ch| ch as u8 - b'0').collect::<Vec<_>>()).collect::<Vec<_>>();
    let low_points = get_low_points(&inp);

    println!("Part one: {}", part_one(&inp, &low_points));
    println!("Part one: {}", part_two(&inp, &low_points));
}
