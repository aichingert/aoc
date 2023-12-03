fn solve(inp: &Vec<&str>) -> (u32, u32) {
    let (mut p1, mut p2) = (0, 0);

    for i in 0..inp.len() {
        let (mut p1_flag, mut red, mut green, mut blue) = (true, 0, 0, 0);
        let (id, vals) = inp[i].split_once(": ").unwrap();
        let sets = vals.split("; ").collect::<Vec<_>>();

        for set in sets {
            let marbles = set.split(", ").collect::<Vec<_>>();

            for marble in marbles {
                let (n, x) = marble.split_once(" ").unwrap();
                let n: u32 = n.parse::<u32>().unwrap();

                match x {
                    "red"   => { red = red.max(n); if n > 12 { p1_flag = false; } },
                    "green" => { green = green.max(n); if n > 13 { p1_flag = false; } },
                    "blue"  => { blue = blue.max(n); if n > 14 { p1_flag = false; } },
                    _ => panic!("unknown color {}", x),
                }
            }
        }

        if p1_flag {
            p1 += id.split_once(" ").unwrap().1.parse::<u32>().unwrap();
        }

        p2 += red * green * blue;
    }

    (p1, p2)
}

fn main() {
    let inp = std::fs::read_to_string("../input/02").unwrap().trim().to_string();
    let (p1, p2) = solve(&inp.lines().collect::<Vec<_>>());

    println!("Part one: {}", p1);
    println!("Part two: {}", p2);
}
