use std::collections::HashMap;

type Point = (i32, i32);

fn part_one(lines: &Vec<(Point, Point)>) -> usize {
    let mut map: HashMap<Point, u16> = HashMap::new();

    lines
        .iter()
        .filter(|(p1, p2)| p1.0 == p2.0 || p1.1 == p2.1)
        .for_each(|(p1, p2)| {
            let minx = p1.0.min(p2.0);
            let maxx = p1.0.max(p2.0);
            let miny = p1.1.min(p2.1);
            let maxy = p1.1.max(p2.1);

            for x in minx..=maxx {
                for y in miny..=maxy {
                    map.entry((x, y)).and_modify(|hit| *hit += 1).or_insert(1);
                }
            }
        });

    map.values().filter(|hit| **hit > 1).count()
}

fn part_two(lines: &Vec<(Point, Point)>) -> usize {
    let mut map = HashMap::new();

    lines
        .iter()
        .for_each(|(p1, p2)| {
            let minx = p1.0.min(p2.0);
            let maxx = p1.0.max(p2.0);
            let miny = p1.1.min(p2.1);
            let maxy = p1.1.max(p2.1);

            if p1.0 == p2.0 || p1.1 == p2.1 {
                for x in minx..=maxx {
                    for y in miny..=maxy {
                        map.entry((x, y)).and_modify(|x| *x += 1).or_insert(1);
                    }
                }
            } else {
                for o in 0..=(minx - maxx).abs() {
                    let x = if p1.0 > p2.0 { p1.0 - o } else { p1.0 + o };
                    let y = if p1.1 > p2.1 { p1.1 - o } else { p1.1 + o };

                    map.entry((x,y)).and_modify(|x| *x += 1).or_insert(1);
                }
            }
        });

    map.values().filter(|hit| **hit > 1).count()
}

fn main() {
    let inp = std::fs::read_to_string("../input/05").unwrap().trim().to_string();
    let inp: Vec<(Point, Point)> = inp.lines().map(|l| {
        let (src, dst) = l.split_once(" -> ").unwrap();
        let (x1,y1) = src.split_once(",").unwrap();
        let (x2,y2) = dst.split_once(",").unwrap();

        ((x1.parse().unwrap(), y1.parse().unwrap()), (x2.parse().unwrap(), y2.parse().unwrap()))
    }).collect::<Vec<_>>();

    println!("Part one: {}", part_one(&inp));
    println!("Part two: {}", part_two(&inp));
}
