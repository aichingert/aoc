pub struct Aoc2022_05 {
    r: Vec<(usize, usize, usize)>,
    s: Vec<Vec<String>>
}
        
impl Aoc2022_05 {
    pub fn new() -> Self {
        Self { r: vec![], s: Vec::new() }
    }
}
        
impl crate::Solution for Aoc2022_05 {
    fn name(&self) -> (usize, usize) {
        (2022, 05)
    }
        
    fn parse(&mut self) {
        // If you are using windows you have to split on \r\n\r\n    
        let input: String = std::fs::read_to_string("input/2022/05.txt").expect("unable to open file!");                                                     
        //                                   | here
        //                                   v
        let input: Vec<&str> = input.split("\n\n").collect();
        //                                                           |                               | here
        //                                                           v                               v
        let (head, body): (Vec<&str>, Vec<&str>) = (input[0].split("\n").collect(), input[1].split("\n").collect());

        for i in 0..head.len() - 1 {
            let cargos: Vec<&str> = head[i].split(' ').collect();
            let (mut i, mut j): (usize, usize) = (0, 0);

            while i < cargos.len() {
                if cargos[i] == "" {
                    i+=4;
                    j+=1;
                    if self.s.len() <= j {
                        self.s.push(vec![]);
                    }
                } else {
                    if self.s.len() > j {
                        self.s[j].push(cargos[i][1..=1].to_string());
                    } else {
                        self.s.push(vec![cargos[i][1..=1].to_string()]);
                    }
                    
                    i += 1;
                    j += 1;
                }
            }
        }

        for i in 0..body.len()-1 {
            let values: Vec<usize> = body[i].split(' ').filter(|s| s.parse::<usize>().is_ok()).map(|s| s.parse().unwrap()).collect();
            self.r.push((values[0], values[1], values[2]));
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut stack: Vec<Vec<String>> = self.s.clone();

        for i in 0..self.r.len() {
            for _amt in 0..self.r[i].0 {
                let v = stack[self.r[i].1-1].remove(0);
                stack[self.r[i].2-1].insert(0, v);
            }
        }

        crate::output(stack.iter().filter(|s| s.len() > 0).map(|s| s[0].as_str()).collect::<String>())
    }
        
    fn part2(&mut self) -> Vec<String> {
        for i in 0..self.r.len() {
            let mut strg: Vec<String> = Vec::new();
            for _amt in 0..self.r[i].0 {
                let v = self.s[self.r[i].1-1].remove(0);
                strg.push(v);
            }

            for j in 0..strg.len() {
                self.s[self.r[i].2-1].insert(0, strg[strg.len() - j - 1].clone());
            }
        }

        crate::output(self.s.iter().filter(|s| s.len() > 0).map(|s| s[0].as_str()).collect::<String>())
    }
}
