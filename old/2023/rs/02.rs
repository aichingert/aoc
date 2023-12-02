fn part_one(inp: &Vec<&str>) -> u32 {
    let mut p = 0;

    'outer: for i in 0..inp.len() {
        let (id, val) = inp[i].split_once(": ").unwrap();

        let sets = val.split("; ").collect::<Vec<_>>();

        for set in sets {
            let marbles = set.split(", ").collect::<Vec<_>>();

            for marble in marbles {
                let (n, x) = marble.split_once(" ").unwrap();
                let n: u32 = n.parse::<u32>().unwrap();

                match x {
                    "red" => if n > 12 { continue 'outer; },
                    "green" => if n > 13 { continue 'outer; },
                    "blue" => if n > 14 { continue 'outer; },
                    _ => panic!("invalid color {}", x),
                }
            }
        }

        p += id.split_once(" ").unwrap().1.parse::<u32>().unwrap();
    }

    p
}

fn part_two(inp: &Vec<&str>) -> u32 {
     let mut p = 0;

    for i in 0..inp.len() {
        let (id, val) = inp[i].split_once(": ").unwrap();
        let (mut red, mut green, mut blue) = (0,0,0);

        let sets = val.split("; ").collect::<Vec<_>>();

        for set in sets {
            let marbles = set.split(", ").collect::<Vec<_>>();

            for marble in marbles {
                let (n, x) = marble.split_once(" ").unwrap();
                let n: u32 = n.parse::<u32>().unwrap();

                match x {
                    "red" => red = red.max(n),
                    "green" => green = green.max(n),
                    "blue" => blue = blue.max(n),
                    _ => panic!("invalid color {}", x),
                }
            }
        }

        p += red * green * blue;
    }

    p
}

fn main() {
    let inp = std::fs::read_to_string("../input/02").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l).collect::<Vec<_>>();

    

    println!("Part one: {}", part_one(&inp));
    println!("Part two: {}", part_two(&inp));
}
