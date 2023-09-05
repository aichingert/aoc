// Advent of Code 2015, day 15
// (c) aichingert

fn solve(ing: &Vec<Vec<i32>>, perms: &Vec<Vec<i32>>, part: bool) -> i32 {
    let mut ans = 0;

    for perm in perms {
        let mut cur = vec![0; ing[0].len()];
        for i in 0..ing.len() {
            for j in 0..ing[i].len() {
                cur[j] += ing[i][j] * perm[i];
            }
        }

        let cal = cur.pop().unwrap();
        if !part && cal != 500 {
            continue;
        }

        ans = ans.max(
            cur.iter()
                .map(|v| if *v > 0 { *v } else { 0 })
                .fold(1, |sum, val| sum * val),
        );
    }

    ans
}

fn parse() -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut ing = Vec::new();
    let mut perms = Vec::new();

    for l in std::fs::read_to_string("../input/15").unwrap().lines() {
        let vls = l.split(' ').collect::<Vec<&str>>();
        ing.push(vec![
            vls[2][..vls[2].len() - 1].parse().unwrap(),
            vls[4][..vls[4].len() - 1].parse().unwrap(),
            vls[6][..vls[6].len() - 1].parse().unwrap(),
            vls[8][..vls[8].len() - 1].parse().unwrap(),
            vls[10].parse().unwrap(),
        ]);
    }

    for i in 1..100 {
        for j in 1..100 {
            if i + j > 100 {
                break;
            }
            for k in 1..100 {
                if i + j + k > 100 {
                    break;
                }
                for l in 1..100 {
                    if i + j + k + l > 100 {
                        break;
                    }
                    if i + j + k + l == 100 {
                        perms.push(vec![i, j, k, l]);
                    }
                }
            }
        }
    }

    (ing, perms)
}

fn main() {
    let (ing, perms) = parse();

    println!("Part 1: {}", solve(&ing, &perms, true));
    println!("Part 2: {}", solve(&ing, &perms, false));
}
