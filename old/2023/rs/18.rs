use std::collections::{HashSet, VecDeque};

fn main() {
    let inp = std::fs::read_to_string("../input/18").unwrap().trim().to_string();
    let inp = inp.split("\n").map(|s| s).collect::<Vec<_>>();

    let mut map = HashSet::from([(0,0)]);
    let mut pos = (0, 0);


    for line in inp {
        let values = line.split(" ").collect::<Vec<_>>();


        let chars = values[2].chars().collect::<Vec<_>>();

        let n = i64::from_str_radix(&chars[2..chars.len() - 2].iter().collect::<String>(), 16).unwrap();

        match chars[7] {
            '3' => {
                for i in 0..n {
                    pos.0 -= 1;
                    map.insert(pos);
                }

            }
            '1' => {
                for i in 0..n {
                    pos.0 += 1;
                    map.insert(pos);
                }

            }
            '2' => {
                for i in 0..n {
                    pos.1 -= 1;
                    map.insert(pos);
                }

            }
            '0' => {
                for i in 0..n {
                    pos.1 += 1;
                    map.insert(pos);
                }

            }
            _ => panic!("invali"),
        }
    }

    for i in 0..10 {
        for j in 0..10 {
            if map.contains(&(i, j)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    let mut bfs = VecDeque::from([(1, 1)]);

    while let Some((y, x)) = bfs.pop_front() {
        if map.contains(&(y, x)) {
            continue;
        }

        map.insert((y, x));

        bfs.push_back((y, x - 1));
        bfs.push_back((y + 1, x));
        bfs.push_back((y, x + 1));
        bfs.push_back((y - 1, x));
    }

    println!("{:?}", map.len());

}

fn fill(map: &mut HashSet<(i32,i32)>, pos: (i32,i32)) {
    if map.contains(&pos) {
        return;
    }

    map.insert(pos);

    fill(map, (pos.0, pos.1 - 1));
    fill(map, (pos.0 + 1, pos.1));
    fill(map, (pos.0, pos.1 + 1));
    fill(map, (pos.0 - 1, pos.1));
}
