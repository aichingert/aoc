use std::collections::HashSet;

#[derive(Debug)]
struct Shape {
    liy: u32,
    points: HashSet<(u32, u32, u32)>,
}

impl Shape {
    fn falling(&mut self, others: &Vec<Self>) {
        loop {
            let mut dropped = HashSet::new();

            for &(x, y, z) in self.points.iter() {
                if y == 0 || Shape::overlaps(others, (x, y - 1, z)) {
                    return;
                }

                dropped.insert((x, y - 1, z));
            }

            self.points = dropped;
        }
    }

    fn overlaps(others: &Vec<Self>, point: (u32, u32, u32)) -> bool {
        for i in 0..others.len() {
            if others[i].points.contains(&point) {
                return true;
            }
        }

        false
    }

    fn holds(others: &Vec<Self>, point: (u32, u32, u32), ignore: usize) -> Vec<usize> {
        let mut shapes = Vec::new();

        for i in 0..others.len() {
            if i != ignore && others[i].points.contains(&point) {
                shapes.push(i);
            }
        }

        shapes
    }
}

fn parse(s: &str) -> u32 {
    s.parse::<u32>().unwrap()
}

fn main() {
    let inp = std::fs::read_to_string("../input/22").unwrap().trim().to_string();
    let inp = inp.lines().map(|line| {
        let (l, r) = line.split_once("~").unwrap();
        (l.split(',').map(parse).collect(), r.split(',').map(parse).collect())
    }).collect::<Vec<(Vec<u32>, Vec<u32>)>>();

    let mut shapes = Vec::new();

    for (lhs, rhs) in inp {
        let mut points = HashSet::new();

        for x in lhs[0]..=rhs[0] {
            for y in lhs[1]..=rhs[1] {
                for z in lhs[2]..=rhs[2] {
                    points.insert((x, y, z));
                }
            }
        }

        shapes.push(Shape { liy: lhs[1], points });
    }

    shapes.sort_by(|a,b| a.liy.cmp(&b.liy));

    let mut i = 0;

    while i < shapes.len() {
        let mut shape = shapes.remove(i);

        shape.falling(&shapes);
        shapes.insert(i, shape);
        
        i += 1;
    }

    let mut ans = 0;

    for i in 0..shapes.len() {
        let mut holds = Vec::new();

        for &(x, y, z) in shapes[i].points.iter() {
            holds.extend_from_slice(&Shape::holds(&shapes, (x, y + 1, z), i));
        }

        let mut takeable = true;

        'dd: for j in 0..holds.len() {
            for &(x, y, z) in shapes[holds[j]].points.iter() {
                if Shape::holds(&shapes, (x, y - 1, z), i).len() > 1 {
                    takeable = false;
                    break 'dd;
                }
            }
        }

        if takeable {
            ans += 1;
        }
    }

    println!("{:?}", ans);

}
