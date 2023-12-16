use std::collections::HashSet;

#[derive(Debug)]
struct Beam {
    y: usize,
    x: usize,
    dir: u8,
}

impl Beam {
    fn new(y: usize, x: usize, dir: u8) -> Self {
        Self {
            y,
            x,
            dir
        }
    }

    fn next(&mut self, c: &HashSet<(usize, usize, u8)>, h: usize, w: usize) -> bool { 
        match self.dir {
            0 => { // east
                if self.x + 1 >= w {
                    return false;
                }

                self.x += 1;
            }
            1 => { // west
                if self.x as i32 - 1 < 0 {
                    return false;
                }
                
                self.x -= 1;
            }
            2 => { // south
                if self.y + 1 >= h {
                    return false;
                }

                self.y += 1;
            }
            3 => { // north
                if self.y as i32 - 1 < 0 {
                    return false;
                }

                self.y -= 1;
            }
            _ => panic!("invalid dir {}", self.dir),
        }

        if c.contains(&(self.y, self.x, self.dir)) {
            return false;
        }

        true
    }
}

fn solve(map: &Vec<Vec<char>>, y: usize, x: usize, dir: u8) -> u32 {
    let mut eng = HashSet::from([(y, x, dir)]);
    let mut beams = vec![Beam::new(y, x, dir)];

    match (beams[0].dir, map[beams[0].y][beams[0].x]) {
        (2, '-') | (3, '-') => {
            beams[0].dir = 0;
            beams.push(Beam::new(beams[0].y, beams[0].x, 1));
        }
        (0, '|') | (1, '|') => {
            beams[0].dir = 2;
            beams.push(Beam::new(beams[0].y, beams[0].x, 3));
        }
        (0, '/') => beams[0].dir = 3,
        (1, '/') => beams[0].dir = 2,
        (2, '/') => beams[0].dir = 1,
        (3, '/') => beams[0].dir = 0,
        (0, '\\') => beams[0].dir = 2,
        (1, '\\') => beams[0].dir = 3,
        (2, '\\') => beams[0].dir = 0,
        (3, '\\') => beams[0].dir = 1,
        _ => (),
    }
    println!("{:?}", beams);

    loop {
        if beams.len() == 0 {
            break;
        }
        let mut rm = Vec::new();

        for i in 0..beams.len() {
            if beams[i].next(&eng, map.len(), map[0].len()) {
                eng.insert((beams[i].y, beams[i].x, beams[i].dir));

                match (beams[i].dir, map[beams[i].y][beams[i].x]) {
                    (2, '-') | (3, '-') => {
                        beams[i].dir = 0;
                        beams.push(Beam::new(beams[i].y, beams[i].x, 1));
                    }
                    (0, '|') | (1, '|') => {
                        beams[i].dir = 2;
                        beams.push(Beam::new(beams[i].y, beams[i].x, 3));
                    }
                    (0, '/') => beams[i].dir = 3,
                    (1, '/') => beams[i].dir = 2,
                    (2, '/') => beams[i].dir = 1,
                    (3, '/') => beams[i].dir = 0,
                    (0, '\\') => beams[i].dir = 2,
                    (1, '\\') => beams[i].dir = 3,
                    (2, '\\') => beams[i].dir = 0,
                    (3, '\\') => beams[i].dir = 1,
                    _ => (),
                }
            } else {
                rm.push(i - rm.len());
            }
        }

        for i in 0..rm.len() {
            beams.remove(rm[i]);
        }
    }

    let mut ans = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if eng.contains(&(i, j, 0)) || eng.contains(&(i, j, 1)) || eng.contains(&(i, j, 2))  || eng.contains(&(i, j, 3)) {
                ans += 1;
            }
        }
    }

    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/16").unwrap().trim().to_string();
    let map = inp.split("\n").map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("{}", solve(&map, 0, 0, 0));

    let mut ans = 0;

    ans = ans.max(solve(&map, 0, 0, 2));
    ans = ans.max(solve(&map, 0, 0, 0));

    ans = ans.max(solve(&map, 0, map[0].len() - 1, 2));
    ans = ans.max(solve(&map, 0, map[0].len() - 1, 1));

    ans = ans.max(solve(&map, map.len() - 1, 0, 3));
    ans = ans.max(solve(&map, map.len() - 1, 0, 0));

    ans = ans.max(solve(&map, map.len() - 1, map[0].len() - 1, 3));
    ans = ans.max(solve(&map, map.len() - 1, map[0].len() - 1, 1));
    for i in 1..map.len() - 1 {
        ans = ans.max(solve(&map, i, 0, 0));
        ans = ans.max(solve(&map, i, map[0].len() - 1, 1));
    }

    for i in 1..map.len() - 1 {
        ans = ans.max(solve(&map, i, 0, 0));
        ans = ans.max(solve(&map, i, map[0].len() - 1, 1));
    }

    for j in 1..map[0].len() - 1 {
        ans = ans.max(solve(&map, 0, j, 2));
        ans = ans.max(solve(&map, map.len() - 1, j, 3));
    }
    
    println!("{:?}", ans);

}
