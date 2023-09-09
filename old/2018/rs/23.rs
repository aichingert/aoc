// Advent of Code 2018, day 23
// (c) aichingert

struct Nano {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}

impl Nano {
    fn new(x: i64, y: i64, z: i64, r: i64) -> Self {
        Self { x, y, z, r }
    }

    fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn is_in_coordinates(&self, pos: (i64, i64, i64)) -> bool {
        (self.x - self.r > pos.0 && self.x + self.r < pos.0)
            && (self.y - self.r > pos.1 && self.y + self.r < pos.1)
            && (self.z - self.r > pos.2 && self.z + self.r < pos.2)
    }

    fn in_range(nanos: &Vec<Self>, pos: &Self, p1: bool) -> usize {
        nanos
            .iter()
            .filter(|n| n.manhattan_distance(pos) <= if p1 { pos.r } else { n.r })
            .count()
    }
}

fn part1(nanos: &Vec<Nano>) -> usize {
    let (mut loc, mut r) = (0, 0);

    for i in 0..nanos.len() {
        if nanos[i].r > r {
            loc = i;
            r = nanos[i].r;
        }
    }

    Nano::in_range(&nanos, &nanos[loc], true)
}

fn part2(nanos: &Vec<Nano>) -> i64 {
    let (mut mix, mut max) = (i64::MAX, i64::MIN);
    let (mut miy, mut may) = (i64::MAX, i64::MIN);
    let (mut miz, mut maz) = (i64::MAX, i64::MIN);
    let mut a = 0;
    let mut d = 0;
    let mut c = (0, 0, 0);

    for i in 0..nanos.len() {
        if nanos[i].x < mix {
            mix = nanos[i].x;
        }
        if nanos[i].x > max {
            max = nanos[i].x;
        }
        if nanos[i].y < miy {
            miy = nanos[i].y;
        }
        if nanos[i].y > may {
            may = nanos[i].y;
        }
        if nanos[i].z < miz {
            miz = nanos[i].z;
        }
        if nanos[i].z > maz {
            maz = nanos[i].z;
        }
    }

    println!("{:?}", max - mix);
    println!("{:?}", may - miy);
    println!("{:?}", maz - miz);

    for x in mix..max {
        for y in miy..may {
            /*
            for z in miz..maz {
                let n = Nano::new(x, y, z, 0);
                let i = Nano::in_range(&nanos, &n, false);

                if i > a {
                    a = i;
                    c = (n.x, n.y, n.z);
                }
            }
            */

            let n = Nano::new(x, y, 0, 0);
            let i = Nano::in_range(&nanos, &n, false);

            if i > a {
                a = i;
                c = (n.x, n.y, n.z);
            }
        }
    }

    println!("{:?}", a);
    println!("{:?}", c);
    0
}

fn parse() -> Vec<Nano> {
    let mut nanos = Vec::<Nano>::new();

    for l in std::fs::read_to_string("../input/23").unwrap().lines() {
        let v = l.split('=').collect::<Vec<&str>>();

        let c = v[1].split(',').collect::<Vec<&str>>();
        nanos.push(Nano::new(
            c[0][1..].parse().unwrap(),
            c[1].parse().unwrap(),
            c[2][..c[2].len() - 1].parse().unwrap(),
            v[2].parse().unwrap(),
        ));
    }

    nanos
}

fn main() {
    let nanos = parse();

    println!("Part 1: {}", part1(&nanos));
    println!("Part 2: {}", part2(&nanos));
}
