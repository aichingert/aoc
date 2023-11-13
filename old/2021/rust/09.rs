fn get_low_points(inp: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
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

fn part_one(inp: &Vec<Vec<u32>>, low_points: &Vec<(usize, usize)>) -> u32 {
    low_points.iter().map(|(i, j)| inp[*i][*j] + 1).sum::<u32>()
}

fn part_two(inp: &Vec<Vec<u32>>, low_points: &Vec<(usize, usize)>) -> u32 {
    0
}

fn main() {
    let inp = std::fs::read_to_string("../input/09").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l.chars().map(|ch| (ch as u8 - b'0') as u32).collect::<Vec<_>>()).collect::<Vec<_>>();
    let low_points = get_low_points(&inp);

    println!("Part one: {}", part_one(&inp, &low_points));
}
