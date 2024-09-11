use std::collections::HashSet;

fn main() {
    let inp = std::fs::read_to_string("../../../input/2015/03").unwrap();

    let (mut x, mut y) = (0, 0);
    let (mut s, mut i) = ([[0, 0], [0, 0]], 0usize);
    let mut p1: HashSet<(i32, i32)> = HashSet::from_iter([(x, y)]);
    let mut p2: HashSet<(i32, i32)> = HashSet::from_iter([(x, y)]);

    for ch in inp.chars() {
        match ch {
            '>' => { 
                x += 1;
                s[i][0] += 1;
            }
            'v' => { 
                y += 1;
                s[i][1] += 1;
            }
            '^' => { 
                y -= 1;
                s[i][1] -= 1;
            }
            '<' => { 
                x -= 1;
                s[i][0] -= 1;
            }
            _ => (),
        }

        p1.insert((x, y));
        p2.insert((s[i][0], s[i][1]));
        i = 1 - i;
    }

    println!("p1: {}", p1.len());
    println!("p2: {}", p2.len());
}
