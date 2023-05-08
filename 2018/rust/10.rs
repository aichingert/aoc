// Advent of Code 2018, day 10
// (c) aichingert

type Cord = (i32,i32);

struct Star {
    pos: Cord,
    vel: Cord,
}

impl Star {
    fn new(pos: Cord, vel: Cord) -> Self {
        Self { pos, vel }
    }

    fn dist(stars: &Vec<Self>) -> (i32,i32,i32,i32) {
        let (mut lx, mut rx, mut by, mut ty) = (i32::MAX,-i32::MAX,i32::MAX,-i32::MAX);

        for i in 0..stars.len() {
            if stars[i].pos.0 < lx { lx = stars[i].pos.0; }
            if stars[i].pos.0 > rx { rx = stars[i].pos.0; }
            if stars[i].pos.1 < by { by = stars[i].pos.1; }
            if stars[i].pos.1 > ty { ty = stars[i].pos.1; }
        }

        (lx,rx,by,ty)
    }

    fn update(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
    }
}

fn solve(stars: &mut Vec<Star>) {
    let mut rng = 0;

    loop {
        for i in 0..stars.len() {
            stars[i].update();
        }

        let (lx,rx,by,ty) = Star::dist(&stars);

        if (lx.abs() - rx.abs()).abs() < 100 && (by.abs() - ty.abs()).abs() < 100 {
            rng += 1;

            for i in lx-3..rx+3 {
                for j in by-3..ty+3 {
                    let mut c = false;
                    for x in 0..stars.len() {
                        if stars[x].pos == (i,j) {
                            c = true;
                            break;
                        }
                    }
                    
                    if c {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
        }

        if rng > 4 {
            break;
        }
    }
}

fn parse() -> Vec<Star> {
    let mut stars = Vec::<Star>::new();
    let inp = std::fs::read_to_string("../input/10").unwrap();
    
    for l in inp.lines() {
        let (a,b) = l.split_once("> ").unwrap();
        let (_,vals) = a.split_once("=<").unwrap();
        let p = vals.split(',').map(|s| s.trim().parse().unwrap()).collect::<Vec<i32>>();
        let (_,vals) = b.split_once("=<").unwrap();
        let (a,b) = vals.split_once(',').unwrap();

        stars.push(Star::new((p[0],p[1]),(a.trim().parse().unwrap(),b[..b.len()-1].trim().parse().unwrap())));
    }

    stars
}

fn main() {
    let mut stars = parse();

    println!("Solution: ");
    solve(&mut stars);
}
