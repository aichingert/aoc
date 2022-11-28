use aoc::read_to_chars;

pub struct Aoc2015_04 {
    d: String
}

impl Aoc2015_04 {
    pub fn new() -> Self {
        Self { d: String::new() }
    }
    
    fn get_hash(s: &str) -> String {
        format!("{:X}", md5::compute(s.as_bytes()))
    }
}

impl crate::Solution for Aoc2015_04 {
    fn name(&self) -> (usize, usize) {
        (2015, 4)
    }

    fn parse(&mut self) {
        self.d = read_to_chars("input/2015/04.txt").into_iter().collect::<String>();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut i = 1;
        loop {
            let s: Vec<char> = Aoc2015_04::get_hash(&(self.d.clone() + &i.to_string())).chars().collect();
             if s[..5].iter().map( | v | match v {
                '0' => 0,
                _ => 1,
            }).sum::<i32>() == 0 {
                break;
            }

            i += 1;
        }

        crate::output(i)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut i = 10000;
        loop {
            let s: Vec<char> = Aoc2015_04::get_hash(&(self.d.clone() + &i.to_string())).chars().collect();
             if s[..6].iter().map( | v | match v {
                '0' => 0,
                _ => 1,
            }).sum::<i32>() == 0 {
                break;
            }

            i += 1;
        }

        crate::output(i)
    }
}
