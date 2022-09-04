use aoc::get_chars;

pub struct Aoc2021_03 {
    d: Vec<Vec<char>>
}

impl Aoc2021_03 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn rem(c: &mut Vec<Vec<char>>, i: usize, rem: char) {
        let mut offset: usize = 0;
    
        for j in 0..c.len() {
            if c[j - offset][i] == rem && c.len() > 1 {
                c.remove(j - offset);
                offset += 1;
            }
        }
    }

    fn evolution(c: &mut Vec<Vec<char>>, o: char, z: char) -> Vec<char> {
        let mut v = [0, 0];
        for i in 0..c[0].len() {
            for j in 0..c.len() {
                match c[j][i] {
                    '0' => v[0] += 1,
                    '1' => v[1] += 1,
                    _ => panic!("invalid char")
                }
            }
    
            if v[0] > v[1] {
                Aoc2021_03::rem(c, i, o);
            } else{
                Aoc2021_03::rem(c, i, z);
            }
    
            v = [0, 0];
        }

        c[0].clone()
    }
}

impl crate::Solution for Aoc2021_03 {
    fn name(&self) -> (usize, usize) {
        (2021, 3)
    }

    fn parse(&mut self) {
        self.d = get_chars("input/2021/03.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        let mut g: String = String::new();
        let mut e: String = String::new();
        for i in 0..self.d[0].len() {
            let mut z: i32 = 0;
            let mut o: i32 = 0;
            for j in 0..self.d.len() {
                match self.d[j][i] {
                    '1' => o += 1,
                    '0' => z+= 1,
                    _ => panic!("invalid input")
                }
            }

            if z > o {
                g.push('0');
                e.push('1');
            } else {
                g.push('1');
                e.push('0');
            }
        }
        crate::output(isize::from_str_radix(&g, 2).expect("invalid gamma string") * isize::from_str_radix(&e, 2).expect("invalid epsilon string"))
    }

    fn part2(&mut self) -> Vec<String> {
        let ogx: Vec<char> = Aoc2021_03::evolution(&mut self.d.clone(), '1', '0');
        let co2: Vec<char> = Aoc2021_03::evolution(&mut self.d.clone(), '0', '1');
        crate::output(isize::from_str_radix(&ogx.into_iter().collect::<String>(), 2).expect("invalid gamma string") * isize::from_str_radix(&co2.into_iter().collect::<String>(), 2).expect("invalid epsilon string"))
    }
}