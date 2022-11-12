use combination::combine::from_vec_at;

const TARGET: i32 = 150;

pub struct Aoc2015_17 {
    containers: Vec<i32>,
    m: usize 
}
        
impl Aoc2015_17 {
    pub fn new() -> Self {
        Self { 
            containers: Vec::new(),
            m: 0
        }
    }
}
        
impl crate::Solution for Aoc2015_17 {
    fn name(&self) -> (usize, usize) {
        (2015, 17)
    }
        
    fn parse(&mut self) {
        self.containers = aoc::read_to_numbers("input/2015/17.txt");
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut c: i32 = 0;

        for i in 0..self.containers.len() {
            for comb in from_vec_at(&self.containers, i) {
                if comb.iter().sum::<i32>() == TARGET {
                    c += 1;

                    if self.m == 0 {
                        self.m = i;
                    }

                }
            }
        }

        crate::output(c)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut c: i32 = 0;

        for comb in from_vec_at(&self.containers, self.m) {
            if comb.iter().sum::<i32>() == TARGET {
                c+=1
            }
        }

        crate::output(c)
    }
}
