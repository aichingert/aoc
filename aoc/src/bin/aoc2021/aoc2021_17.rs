pub struct Aoc2021_17 {
    a_x: Vec<i32>,
    a_y: Vec<i32>,
}
        
impl Aoc2021_17 {
    pub fn new() -> Self {
        Self { 
            a_x: Vec::new(),
            a_y: Vec::new()
        }
    }
}
        
impl crate::Solution for Aoc2021_17 {
    fn name(&self) -> (usize, usize) {
        (2021, 17)
    }
        
    fn parse(&mut self) {
        let i: String = std::fs::read_to_string("input/2021/17.txt").expect("unable to open file").trim().to_string();
        let p: Vec<&str> = i.split(' ').collect();
        let mut e: Vec<&str> = p[2].split('=').collect();
        let c: Vec<&str> = e[1].split(',').collect();
        let mut v: Vec<&str> = c[0].split("..").collect();

        let mut f: i32 = v[0].parse::<i32>().unwrap();
        let mut t: i32 = v[1].parse::<i32>().unwrap();

        self.a_x = (f..=t).collect();

        e = p[3].split('=').collect();
        v = e[1].split("..").collect();

        f = v[0].parse::<i32>().unwrap();
        t = v[1].parse::<i32>().unwrap();

        self.a_y = (f..=t).collect();
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut m: i32 = 0;

        for x in -500..500 {
            for y in -500..500 {
                let p: f32 = x as f32 * (1f32 + x as f32) / 2f32;

                if self.a_x.contains(&(p as i32)) {
                    let m_y = (y as f32 * (1f32 + y as f32) / 2f32) as i32;
                    let mut y_v = y;
                    let mut y_p = y;

                    while y_p > self.a_y[0] {
                        y_p += y_v;
                        y_v -= 1;
                    }

                    if self.a_y.contains(&y_p) && m_y > m {
                        m = m_y;
                    }
                }
            }
        }


        crate::output(m)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut c: i32 = 0;

        for x in -500..500 {
            for y in -500..500 {
                let mut x_v = x;
                let mut x_p = x;
                let mut y_v = y;
                let mut y_p = y;

                while y_p >= self.a_y[0] {
                    if self.a_x.contains(&x_p) && self.a_y.contains(&y_p) {
                        c += 1;
                        break
                    }

                    x_p += x_v;

                    if x_v > 0 {
                        x_v -= 1
                    } else if x_v < 0 {
                        x_v += 1
                    }

                    y_p += y_v;
                    y_v -= 1;
                }
            }
        }

        crate::output(c)
    }
}
