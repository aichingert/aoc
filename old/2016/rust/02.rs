// Advent of Code 2016, day 2
// (c) aichingert

const F: &[[char; 3];3] = &[['1','2','3'],['4','5','6'],['7','8','9']];
const S: &[[char; 5];5] = &[['0','0','1','0','0'], ['0','2','3','4','0'], ['5','6','7','8','9'], ['0','A','B','C','0'], ['0','0','D','0','0']];

fn part1(inp: &Vec<&str>) -> String {
    let mut ans = String::new();
    let (mut x, mut y) = (1, 1);

    for l in inp {
        l.chars().for_each(|ch| {
            match ch {
                'U' => y = i32::max(0, y-1),
                'D' => y = i32::min(2, y+1),
                'R' => x = i32::min(2, x+1),
                _ => x = i32::max(0, x-1)
            };
        });
        ans.push(F[y as usize][x as usize]);
    }

    ans
}

fn part2(inp: &Vec<&str>) -> String {
    let mut ans = String::new();
    let (mut x, mut y) = (0,2);

    for l in inp {
        l.chars().for_each(|ch| {
            match ch {
                'U' => if y>0 && S[y-1][x] != '0' { y -= 1; }
                'D' => if y<4 && S[y+1][x] != '0' { y += 1; }
                'L' => if x>0 && S[y][x-1] != '0' { x -= 1; }
                _ => if x<4 && S[y][x+1] != '0' { x += 1; }
            };
        });
        ans.push(S[y][x]);
    }

    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/02").unwrap();
    let inp = inp.lines().collect::<Vec<&str>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
