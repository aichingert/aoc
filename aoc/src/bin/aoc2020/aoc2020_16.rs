pub struct Aoc2020_16 {
    ticket: Vec<i32>,
    valid_numbers: Vec<i32>,
    nearby_tickets: Vec<Vec<i32>>
}

impl Aoc2020_16 {
    pub fn new() -> Self {
        Self { 
            ticket: vec![],
            valid_numbers: vec![],
            nearby_tickets: vec![]    
        }
    }
}

impl crate::Solution for Aoc2020_16 {
    fn name(&self) -> (usize, usize) {
        (2020, 16)
    }

    fn parse(&mut self) {
        let binding = std::fs::read_to_string("input/2020/16.txt").expect("unable to open file");
        let lines: Vec<&str> = binding.split("\r\n").collect();
        let mut idx: usize = 0;

        while idx < lines.len() {
            if lines[idx] == "" {
                idx+=5;
                break;
            }

            let sec: Vec<&str> = lines[idx].split(": ").collect();
            let both: Vec<&str> = sec[1].split(" or ").collect();
            let mut bound: Vec<i32> = both[0].split('-').map(|s| s.parse::<i32>().unwrap()).collect();
            bound.append(&mut both[1].split('-').map(|s| s.parse::<i32>().unwrap()).collect());

            for i in bound[0]..=bound[1] {
                self.valid_numbers.push(i);
            }
            for i in bound[2]..=bound[3] {
                self.valid_numbers.push(i);
            }

            idx+=1;
        }

        self.ticket = lines[idx-3].split(',').map(|s| s.parse::<i32>().unwrap()).collect();

        while idx < lines.len() {
            self.nearby_tickets.push(lines[idx].split(',').filter(|e| !e.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect());
            idx += 1;
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut idx: usize = 0;
        let mut sum: i32 = 0;

        while idx < self.nearby_tickets.len() {
            let n_sum = self.nearby_tickets[idx].iter().map(|n| 
                if self.valid_numbers.contains(n) {
                    0
                } else {
                    *n
                }).sum::<i32>();
            if n_sum > 0 {
                sum += n_sum;
                self.nearby_tickets.remove(idx);
            } else {
                idx += 1;
            }
        }
        crate::output(sum)
    }

    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}