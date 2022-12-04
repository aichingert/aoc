pub struct Aoc2022_04 {
    r: Vec<Vec<(u32, u32)>>
}
        
impl Aoc2022_04 {
    pub fn new() -> Self {
        Self { 
            r: Vec::new()
        }
    }
}
        
impl crate::Solution for Aoc2022_04 {
    fn name(&self) -> (usize, usize) {
        (2022, 04)
    }
        
    fn parse(&mut self) {
        let input: Vec<Vec<String>> = aoc::read_to_slice("input/2022/04.txt", ",");

        for i in 0..input.len() {
            let f: Vec<u32> = input[i][0].split('-').map(|s| s.parse::<u32>().expect("invalid input!")).collect();
            let s: Vec<u32> = input[i][1].split('-').map(|s| s.parse::<u32>().expect("invalid input!")).collect();

            self.r.push(vec![(f[0], f[1]), (s[0],s[1])]);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.r
                      .iter()
                      .map(|v| 
                           if (v[0].0 <= v[1].0 && v[0].1 >= v[1].1) || (v[1].0 <= v[0].0 && v[1].1 >= v[0].1) { 1 } else { 0 })
                      .sum::<u32>())
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output(self.r
                .iter()
                .map(|v| 
                    if (v[0].1 >= v[1].0 && v[0].0 <= v[1].0) || (v[0].0 <= v[1].1 && v[0].1 >= v[1].0) { 1 } else { 0 })
                .sum::<u32>())
    }
}
