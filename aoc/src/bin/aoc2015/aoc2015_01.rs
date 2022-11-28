use aoc::read_to_chars;

#[derive(Debug)]
pub struct Aoc2015_01 {
    pub d: Vec<char>,
}

impl Aoc2015_01 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}

impl crate::Solution for Aoc2015_01 {
    fn name(&self) -> (usize, usize) {
        (2015, 1)
    }

    fn parse(&mut self) {
        self.d = read_to_chars("input/2015/01.txt");
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(
            self.d.iter().map( | h | match h {
                '(' => 1,
                ')' => -1,
                _ => {
                    println!("{:?}", h);
                    panic!("invalid char in input!");
                }
            }).sum::<i32>()
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut h: i32 = 0;
        for (p, c) in self.d.iter().enumerate() {
            match c {
                '(' => h+=1,
                ')' => h-=1,
                _ => panic!("invalid char in input!"),
            }

            if h < 0 {
                return crate::output(p + 1);
            }
        }

        panic!("no answer found!");
    }
}
