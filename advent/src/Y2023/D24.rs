#[derive(Debug)]
struct Linear {
    k: f64,
    d: f64,
    _x: f64,
}

impl Linear {
    fn new(pos: Vec<f64>, vel: Vec<f64>) -> Self {
        let k = ((pos[1] + vel[1]) - pos[1]) / ((pos[0] + vel[0]) - pos[0]);
        let d = (k * pos[0] - pos[1]) * (-1.);
        let x = (pos[1] - d) / k;

        Self {
            k,
            d,
            _x: x,
        }
    }

    fn intersect(&self, other: &Self) -> Option<(f64, f64)> {
        if other.d - self.d == 0. || self.k - other.k == 0. {
            return None;
        }

        let x = (other.d - self.d) / (self.k - other.k);
        let y = self.k * x + self.d;

        Some((x, y))
    }
}

pub fn solve() {
    let inp = std::fs::read_to_string("input/2023/24").unwrap().trim().to_string();

    let mut functions = Vec::new();

    for line in inp.lines() {
        let (pos, vel) = line.split_once(" @ ").unwrap();

        let pos = pos.split(", ").map(|n| n.trim().parse::<f64>().unwrap()).collect::<Vec<_>>();
        let vel = vel.split(", ").map(|n| n.trim().parse::<f64>().unwrap()).collect::<Vec<_>>();

        functions.push(Linear::new(pos, vel));
    }


    // \        /
    //  \      /
    //   \    /
    //    \  /
    //     \/
    //     /\

    let mut inside = 0;

    let lb = 200000000000000.;
    let ub = 400000000000000.;

    for i in 0..functions.len() {
        for j in i+1..functions.len() {
            if let Some((x, y)) = functions[i].intersect(&functions[j]) {
                if x >= lb && x <= ub && y >= lb && y <= ub {
                    inside += 1;
                }
            }
        }
    }
    
    // ans > 16971
    println!("{:?}", inside);
}
