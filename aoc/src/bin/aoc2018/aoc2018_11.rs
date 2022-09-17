pub struct Aoc2018_11 {
    number: i32
}
        
impl Aoc2018_11 {
    pub fn new() -> Self {
        Self { number: 0 }
    }
}
        
impl crate::Solution for Aoc2018_11 {
    fn name(&self) -> (usize, usize) {
        (2018, 11)
    }
        
    fn parse(&mut self) {
        self.number = 3214;
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut map: [[i32; 300]; 300] = [[0; 300]; 300];
        let mut best: i32 = 0;
        let mut cords: (i32, i32) = (0, 0);

        for i in 1..=300 {
            for j in 1..= 300 {
                let rack_id: i32 = i + 10;
                let p1: i32 = rack_id * j;
                let p2: i32 = p1 + 3214;
                let p3: i32 = p2 * rack_id;
                let s: String = p3.to_string();
                let mut np: i32 = -5;
        
                if s.len() > 3 {
                    np = s[s.len() - 3..=s.len() - 3].parse::<i32>().unwrap() - 5;
                }

                map[i as usize - 1][j as usize - 1] = np;
            }
        }

        for i in 0..map.len() - 2 {
            for j in 0..map[i].len() - 2 {
                let current: i32 = 
                map[i][j + 0] + map[i + 1][j + 0] + map[i + 2][j + 0] +
                map[i][j + 1] + map[i + 1][j + 1] + map[i + 2][j + 1] + 
                map[i][j + 2] + map[i + 1][j + 2] + map[i + 2][j + 2];

                if current > best {
                    best = current;
                    cords = (i as i32, j as i32);
                }
            }
        }

        crate::output(format!("{},{}", cords.0 + 1, cords.1 + 1))
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}