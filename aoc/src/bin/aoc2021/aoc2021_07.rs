pub struct Aoc2021_07 {
    d: Vec<i32>
}
        
impl Aoc2021_07 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn allign(&self, part: bool) -> i32 {
        let mut m: i32 = i32::MAX;

        for height in 0..500 {
            let mut fuel: i32 = 0;

            for i in 0..self.d.len() {
                if !part {
                    fuel += (self.d[i] - height).abs();
                } else {
                    let dist: f32 = (self.d[i] - height).abs() as f32;
                    fuel += (dist / 2.0 * (2.0 + dist-1.0)) as i32;
                }
            }

            m = std::cmp::min(m, fuel);
        }

        m
    }
}
        
impl crate::Solution for Aoc2021_07 {
    fn name(&self) -> (usize, usize) {
        (2021, 07)
    }
        
    fn parse(&mut self) {
        self.d = aoc::read_number_stream("input/2021/07.txt", ",");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output(self.allign(false))
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output(self.allign(true))
    }
}
