pub struct Aoc2020_13 {
    dep: u32,
    b_ids: Vec<u32>,
}
        
impl Aoc2020_13 {
    pub fn new() -> Self {
        Self { 
            dep: 0,
            b_ids: Vec::new()
         }
    }
}
        
impl crate::Solution for Aoc2020_13 {
    fn name(&self) -> (usize, usize) {
        (2020, 13)
    }
        
    fn parse(&mut self) {
        let input = aoc::read_to_slice("input/2020/13.txt", ",");
        self.dep = input[0][0].parse::<u32>().unwrap();
        self.b_ids = input[1].iter().filter(|s| s.parse::<u32>().is_ok()).map(|ok| ok.parse::<u32>().unwrap()).collect();
    }
        
    fn part1(&mut self) -> Vec<String> {
        let (mut earl,mut b_id): (u32, u32) = (u32::MAX, 0);

        self.b_ids.iter().for_each(|id| {
            let res = self.dep / id + 1;

            if res*id-self.dep < earl {
                earl = res*id-self.dep;
                b_id = *id;
            }
        });

        crate::output((self.dep - (self.dep % b_id) + b_id - self.dep) * b_id)
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}