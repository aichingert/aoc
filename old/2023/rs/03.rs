use std::collections::HashMap;

fn part_one(hm: &HashMap<(usize, usize), Vec<u32>>) -> u32 {
    hm.values().map(|vec| vec.iter().sum::<u32>()).sum::<u32>()
}

fn part_two(inp: &Vec<Vec<char>>, hm: &HashMap<(usize, usize), Vec<u32>>) -> u32 {
    hm.keys().filter(|(i, j)| inp[*i][*j] == '*')
        .map(|k| {
            if let Some(l) = hm.get(k).and_then(|l: &Vec<u32>| if l.len() != 2 { None } else { Some(l) }) {
                l[0] * l[1]
            } else { 0 }
        })
        .sum::<u32>()
}

fn parse() -> (Vec<Vec<char>>, HashMap<(usize, usize), Vec<u32>>) {
    let inp = std::fs::read_to_string("../input/03").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut hm = HashMap::new();

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if !inp[i][j].is_digit(10) && inp[i][j] != '.' {
                hm.insert((i, j), extract_numbers_around(&inp, i, j));
            }
        }
    }

    (inp, hm)
}

fn extract_numbers_around(inp: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<u32> {
    let mut seen = Vec::new();
    let mut nums = Vec::new();

    for (x,y) in [(-1,0),(0,-1),(1,0),(0,1),(1,1),(1,-1),(-1,-1),(-1,1)] {
        let (nx, ny) = (j as i32 + x, i as i32 + y);

        if seen.contains(&(ny, nx)) || (nx < 0 || ny < 0 || ny >= inp.len() as i32 || nx >= inp[i].len() as i32) {
            continue;
        }
        let (nx, ny) = (nx as usize, ny as usize);

        if inp[ny][nx].is_digit(10) {
            let (mut lhs, mut rhs) = (nx as i32, nx);

            while rhs + 1 < inp[i].len() && inp[ny][rhs + 1].is_digit(10) {
                seen.push((ny as i32, rhs as i32 + 1));
                rhs += 1;
            }

            while lhs - 1 >= 0 && inp[ny][lhs as usize - 1].is_digit(10) {
                seen.push((ny as i32, lhs - 1));
                lhs -= 1;
            }

            nums.push(inp[ny][lhs as usize..=rhs].iter().collect::<String>().parse::<u32>().unwrap());
        }
    }

    nums
}

fn main() {
    let (inp, hm) = parse();

    println!("Part one: {}", part_one(&hm));
    println!("Part two: {}", part_two(&inp, &hm));
}
