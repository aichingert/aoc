use aoc::slice;

pub struct Aoc2016_02 {
    d: Vec<String>
}

impl Aoc2016_02 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn check_bound(m: &[Vec<char>], x: i32, y: i32) -> bool {
        if y >= m.len() as i32 || y < 0 || x >= m.len() as i32 || x < 0 {
            return false;
        }

        match 5 - m[y as usize].len() {
            4 => if x != 2 { return false } else { return true },
            2 => if x >= 1 && x <= 3 { return true } else { return false },
            0 => true,
            _ => panic!("wrong math")
        }
    }
}

impl crate::Solution for Aoc2016_02 {
    fn name(&self) -> (usize, usize) {
        (2016, 2)
    }

    fn parse(&mut self) {
        self.d = slice("input/2016/02.txt", "\r\n");
    }

    fn part1(&mut self) -> Vec<String> {
        let m = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
        let mut o = String::new();
        let mut x: usize = 0;
        let mut y: usize = 0;

        for i in 0..self.d.len() {
            for j in 0..self.d[i].len() {
                match &self.d[i][j..=j] {
                    "U" => y -= if y as i32 - 1 >= 0 { 1 } else { 0 },
                    "D" => y += if y + 1 < 3 { 1 } else { 0 },
                    "R" => x += if x + 1 < 3 { 1 } else { 0 },
                    "L" => x -= if x as i32 - 1 >= 0 { 1 } else { 0 },
                    _ => panic!("invalid input"),
                }
            }

            o.push(m[y][x])
        }

        crate::output(o)
    }

    fn part2(&mut self) -> Vec<String> {
        let m = [vec!['1'], vec!['2', '3', '4'], vec!['5', '6', '7', '8', '9'], vec!['A', 'B', 'C'], vec!['D']];
        let mut o = String::new();
        let mut x: i32 = 0;
        let mut y: i32 = 2;

        for i in 0..self.d.len() {
            for j in 0..self.d[i].len() {
                match &self.d[i][j..=j] {
                    "U" => y -= if Aoc2016_02::check_bound(&m, x, y - 1) { 1 } else { 0 },
                    "D" => y += if Aoc2016_02::check_bound(&m, x, y + 1) { 1 } else { 0 },
                    "R" => x += if Aoc2016_02::check_bound(&m, x + 1, y) { 1 } else { 0 },
                    "L" => x -= if Aoc2016_02::check_bound(&m, x - 1, y) { 1 } else { 0 },
                    _ => panic!("invalid input"),
                }
            }

            let f = match 5 - m[y as usize].len() {
                4 => { 2 },
                2 => { 1 },
                _ => { 0 }
            };

            o.push(m[y as usize][x as usize - f])
        }

        crate::output(o)
    }
}