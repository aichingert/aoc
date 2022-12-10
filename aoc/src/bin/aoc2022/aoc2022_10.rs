pub struct Aoc2022_10 {
    d: Vec<(String, i32)>
}
        
impl Aoc2022_10 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2022_10 {
    fn name(&self) -> (usize, usize) {
        (2022, 10)
    }
        
    fn parse(&mut self) {
        //let input: String = std::fs::read_to_string("input/2022/10.txt").expect("unable to open file!");
        let input: Vec<Vec<String>> = aoc::read_to_slice("input/2022/10.txt", " ");

        for i in 0..input.len() {
            if input[i].len() > 1 {
                self.d.push((input[i][0].clone(), input[i][1].parse::<i32>().unwrap()));
            } else {
                self.d.push((input[i][0].clone(), 0));
            }
        }

    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut cycle: i32 = 0;
        let mut idx: usize = 0;
        let mut x: i32 = 1;
        let mut sum: i32 = 0;

        while cycle <= 220 {
            let mut current_cycle: i32 = match self.d[idx].0.as_str() {
                "noop" => 1,
                "addx" => 2,
                _ => panic!()
            };

            while current_cycle > 0 {
                cycle += 1;
                current_cycle -= 1;
                if (cycle-20)%40 == 0 {
                    sum += x * cycle;
                    println!("{x} * {cycle}");
                }
            }

            x += match self.d[idx].0.as_str() {
                "noop" => 0,
                "addx" => self.d[idx].1,
                _ => panic!()
            };

            idx += 1;
        }

        crate::output(sum)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut cycle: usize = 0;
        let mut idx: usize = 0;
        let mut x: usize = 0;
        let mut sprite: Vec<char> = vec!['.';40*6]; 

        while cycle < 240 {
            let mut current_cycle: i32 = match self.d[idx].0.as_str() {
                "noop" => 1,
                "addx" => 2,
                _ => panic!()
            };

            while current_cycle > 0 {
                if cycle%40 >= x && cycle%40 <= x+2 {
                    sprite[cycle] = '#';
                }
                cycle += 1;
                current_cycle -= 1;
            }

            x += match self.d[idx].0.as_str() {
                "noop" => 0,
                "addx" => self.d[idx].1 as usize,
                _ => panic!()
            };

            idx += 1;
        }

        for i in 0..6 {
            for j in 0..40 {
                print!("{}", sprite[j+40*i]);
            }
            println!();
        }


        crate::output("Above")
    }
}