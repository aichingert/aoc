use std::collections::HashSet;

pub struct Aoc2022_09 {
    d: Vec<(char, i32)>
}
        
impl Aoc2022_09 {
    pub fn new() -> Self {
        Self { d: Vec::new() }
    }
}
        
impl crate::Solution for Aoc2022_09 {
    fn name(&self) -> (usize, usize) {
        (2022, 09)
    }
        
    fn parse(&mut self) {
        let input: String = std::fs::read_to_string("input/2022/09.txt").expect("unable to open file!");
        
        for line in input.lines() {
            let s: Vec<_> = line.split(' ').collect();

            self.d.push((s[0].chars().next().unwrap(), s[1].parse::<i32>().unwrap()));
        }
        

    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut positions: HashSet<(i32, i32)> = HashSet::new();
        let mut hx: i32 = 0;
        let mut hy: i32 = 0;
        let mut tx: i32 = 0;
        let mut ty: i32 = 0;

        for i in 0..self.d.len() {
            for _ in 0..self.d[i].1 {
                positions.insert((tx, ty));
                match self.d[i].0 {
                    'R' => hx += 1,
                    'L' => hx -= 1,
                    'U' => hy += 1,
                    'D' => hy -= 1,
                    _ => panic!()
                }

                if hx == tx && hy.abs_diff(ty) > 1 {
                    match self.d[i].0 {
                        'R' => tx += 0,
                        'L' => tx -= 0,
                        'U' => ty += 1,
                        'D' => ty -= 1,
                        _ => panic!("same x")
                    }
                } else if hy == ty && hx.abs_diff(tx) > 1 {
                    match self.d[i].0 {
                        'R' => tx += 1,
                        'L' => tx -= 1,
                        'U' => ty += 0,
                        'D' => ty -= 0,
                        _ => panic!("same y")
                    }
                } else if hx != tx && hy != ty && (hx.abs_diff(tx) > 1 || hy.abs_diff(ty) > 1) {
                    match self.d[i].0 {
                        'R' => {
                            ty = hy;
                            tx += 1;
                        },
                        'L' => {
                            ty = hy;
                            tx -= 1;
                        },
                        'U' => {
                            tx = hx;
                            ty += 1;
                        },
                        'D' => {
                            tx = hx;
                            ty -= 1;
                        },
                        _ => panic!()
                    }
                }

                print!("{hx} {hy} -> ");
                println!("{tx} {ty}\n");
            }
        }

        crate::output(positions.len())
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}