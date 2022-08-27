fn main() {
    let inp: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&inp);
    solve_part_two(&inp);
}

#[derive(Debug)]
struct B {
    a: [[(i32, bool); 5]; 5]
}

impl B {
    fn new(b: [[(i32, bool); 5]; 5]) -> Self {
        Self { a: b }
    }

    fn bingo(&self) -> bool {
        let mut h: bool;
        let mut d: bool;

        for i in 0..self.a.len() {
            h = true;
            d = true;
            for j in 0..self.a[i].len() {
                if !self.a[i][j].1 {
                    h = false;
                }

                if !self.a[j][i].1 {
                    d = false;
                }

                if !h && !d {
                    break;
                }
            }

            if h || d {
                return true;
            }
        }

        false
    }
}

fn g(inp: &String) -> (Vec<B>, Vec<i32>) {
    let mut p: Vec<&str> = inp
        .lines()
        .collect();

    let n: Vec<i32> = p
        .remove(0)
        .split(',')
        .map( | s | s.parse::<i32>()
        .unwrap())
        .collect();
    p.remove(0);

    let mut b: Vec<B> = vec![];
    let mut a: [[(i32, bool); 5]; 5] = [[(0, false); 5]; 5];
    let mut y: usize = 0;

    for i in 0..p.len() {
        if p[i] == "" {
            b.push(B::new(a));
            y = 0;
            continue;
        }


        let t: Vec<&str> = p[i].split(' ').collect();
        let mut v: Vec<i32> = vec![];

        for j in 0..t.len() {
            if t[j] != "" {
                v.push(t[j].parse::<i32>().unwrap());
            }
        }

        for j in 0..v.len() {
            a[y][j].0 = v[j];
        }

        y += 1;
    }

    (b, n)
}

fn solve_part_one(inp: &String) {
    let g = g(inp);
    let mut b: Vec<B> = g.0;
    let n: Vec<i32> = g.1;
    let mut r: i32 = 0;

    for i in 0..n.len() {
        for j in 0..b.len() {
            for k in 0..b[j].a.len() {
                for l in 0..b[j].a[k].len() {
                    if b[j].a[k][l].0 == n[i] {
                        b[j].a[k][l].1 = true;
                    }
                }
            }

            if B::bingo(&b[j]) {
                for k in 0..b[j].a.len() {
                    for l in 0..b[j].a[k].len() {
                        if !b[j].a[k][l].1 {
                            r += b[j].a[k][l].0;
                        }
                    }
                }
                r *= n[i];

                break;
            }
        }

        if r != 0 {
            break;
        }
    }

    println!("Solution part one: {}", r);
}

fn solve_part_two(inp: &String) {
    let g = g(inp);
    let mut b: Vec<B> = g.0;
    let n: Vec<i32> = g.1;
    let mut r: i32 = 0;

    'main: for i in 0..n.len() {
        let mut j = 0;

        while j < b.len() {
            for k in 0..b[j].a.len() {
                for l in 0..b[j].a[k].len() {
                    if b[j].a[k][l].0 == n[i] {
                        b[j].a[k][l].1 = true;
                    }
                }
            }
    
            if B::bingo(&b[j]) {
                if b.len() > 1 {
                    b.remove(j);
                    continue;
                } else {
                    for k in 0..b[j].a.len() {
                        for l in 0..b[j].a[k].len() {
                            if !b[j].a[k][l].1 {
                                r += b[j].a[k][l].0;
                            }
                        }
                    }

                    r *= n[i];
                    break 'main;
                }
            }

            j+=1;
        }
    }



    println!("Solution part one: {:?}", r);
}