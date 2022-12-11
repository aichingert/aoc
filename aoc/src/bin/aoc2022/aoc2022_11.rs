#[derive(Debug, Clone)]
pub struct Aoc2022_11 {
    d: Vec<Monkey>,
    lcm: usize
}

#[derive(Debug, Clone)]
struct Monkey {
    id: u32,
    items: Vec<usize>,
    op: (String, String, String),
    test: usize,
    throw: (usize, usize),
    times: usize
}

impl Monkey {
    fn new() -> Self {
        Self {
            id: 0,
            items: Vec::new(),
            op: (String::new(), String::new(), String::new()),
            test: 0,
            throw: (0, 0),
            times: 0
        }
    }
}
        
fn result<'a>(a: usize, b: usize, op: &'a str) -> usize {
    match op {
        "+" => a + b,
        "*" => a * b,
        _ => panic!("invalid operation!")
    }
}

impl Aoc2022_11 {
    pub fn new() -> Self {
        Self {
            d: Vec::new(),
            lcm: 1
        }
    }

    fn calc(&mut self, i: usize) {
        let op: &str = self.d[i].op.1.as_str(); // sets the operation * or +
        
        self.d[i].items[0] = if self.d[i].op.0.as_str() == "old" && self.d[i].op.2.as_str() == "old" {  // handels => old - old
            result(self.d[i].items[0], self.d[i].items[0], op)
        } else if self.d[i].op.0.as_str() == "old" {                                                    // handels => old - level
            result(self.d[i].items[0], self.d[i].op.2.parse::<usize>().unwrap(), op)
        } else {                                                                                        // handels => level - old
            result(self.d[i].items[0], self.d[i].op.0.parse::<usize>().unwrap(), op)
        };
    }

    fn solve(&mut self, part: bool, range: i32) -> usize {
        for _ in 0..range {
            for i in 0..self.d.len() {
                while self.d[i].items.len() > 0 {
                    self.calc(i);
                    
                    let item  = if part {self.d[i].items.remove(0) / 3} else {self.d[i].items.remove(0) % self.lcm};
                    let idx: usize = if item % self.d[i].test == 0 {self.d[i].throw.0} else {self.d[i].throw.1};
                    self.d[idx].items.push(item);
                    self.d[i].times += 1;
                }
            }
        }

        let (mut f, mut s) = (0, 0);

        for i in 0..self.d.len() {
            if self.d[i].times > f {
                s = f;
                f = self.d[i].times;
            } else if self.d[i].times > s {
                s = self.d[i].times;
            }
        }

        f*s
    }
}

impl crate::Solution for Aoc2022_11 {
    fn name(&self) -> (usize, usize) {
        (2022, 11)
    }

    fn parse(&mut self) {
        for monkey in std::fs::read_to_string("input/2022/11.txt").expect("unable to open file!").split("\r\n\r\n").collect::<Vec<&str>>() {
            let lines: Vec<&str> = monkey.split("\r\n").collect();
            let mut new_monkey: Monkey = Monkey::new();
            let (id, items, op, test, t, f) = (
                lines[0], 
                lines[1].split("Starting items: ").collect::<Vec<&str>>(), 
                lines[2].split(' ').collect::<Vec<&str>>(), 
                lines[3].split(' ').filter(|s| s.parse::<usize>().is_ok()).map(|s| s.parse().unwrap()).collect::<Vec<usize>>(), 
                lines[4].split("If true: throw to monkey ").collect::<Vec<&str>>(), 
                lines[5].split("If false: throw to monkey ").collect::<Vec<&str>>());
            new_monkey.id = id[id.len()-2..id.len()-1].parse().unwrap();
            new_monkey.items = items[1].split(", ").map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
            new_monkey.op = (op[5].to_string(), op[6].to_string(), op[7].to_string());
            new_monkey.test = test[0];
            self.lcm = (self.lcm * new_monkey.test) / aoc::gcd(self.lcm, new_monkey.test);
            new_monkey.throw = (t[1].parse().unwrap(), f[1].parse().unwrap());
            self.d.push(new_monkey);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output(self.clone().solve(true, 20))
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output(self.solve(false, 10000))
    }
}