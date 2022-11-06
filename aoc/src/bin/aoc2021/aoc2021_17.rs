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
        crate::output("")
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}
