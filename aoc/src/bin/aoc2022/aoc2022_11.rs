pub struct Aoc2022_11 {
    d: Vec<Monkey>
}

#[derive(Debug)]
struct Monkey {
    id: u32,
    items: Vec<u64>,
    op: (String, String),
    kind: String,
    test: u64,
    throw: (usize, usize),
    times: u32
}

impl Monkey {
    fn new() -> Self {
        Self {
            id: 0,
            items: Vec::new(),
            op: (String::new(), String::new()),
            kind: String::new(),
            test: 0,
            throw: (0, 0),
            times: 0
        }
    }
}
        
impl Aoc2022_11 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }
}
        
impl crate::Solution for Aoc2022_11 {
    fn name(&self) -> (usize, usize) {
        (2022, 11)
    }
        
    fn parse(&mut self) {
        let input: String = std::fs::read_to_string("input/2022/11.txt").expect("unable to open file!");
        let monkeys: Vec<&str> = input.split("\r\n\r\n").collect();

        for i in 0..monkeys.len() {
            let lines: Vec<&str> = monkeys[i].split("\r\n").collect();
            let mut monkey: Monkey = Monkey::new();
            let id: Vec<_> = lines[0].split("Monkey ").collect();
            monkey.id = id[1][0..=0].parse::<u32>().unwrap();
            let items = lines[1].split("Starting items: ").collect::<Vec<&str>>();
            let items: Vec<u64> = items[1].split(", ").collect::<Vec<&str>>().iter().map(|s| s.parse().unwrap()).collect();
            monkey.items = items;
            let op: Vec<&str> = lines[2].split("Operation: new = ").collect();
            let op: Vec<&str> = op[1].split(' ').collect();
            monkey.kind = op[1].to_string();
            monkey.op = (op[0].to_string(), op[2].to_string());
            let test: Vec<&str> = lines[3].split("Test: divisible by ").collect();
            monkey.test = test[1].parse().unwrap();
            let throw: Vec<_> = lines[4].split("If true: throw to monkey ").collect();
            let f: usize = throw[1].parse::<usize>().unwrap();
            let throw: Vec<_> = lines[5].split("If false: throw to monkey ").collect();
            monkey.throw = (f, throw[1].parse::<usize>().unwrap());
            self.d.push(monkey);
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        for _ in 0..20 {
            for i in 0..self.d.len() {
                while self.d[i].items.len() > 0 {
                    match self.d[i].kind.as_str() {
                        "*" => {
                            if self.d[i].op.0.as_str() == "old" && self.d[i].op.1.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].items[0] * self.d[i].items[0];
                            } else if self.d[i].op.0.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].op.1.parse::<u64>().unwrap() * self.d[i].items[0];
                            } else if self.d[i].op.1.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].op.0.parse::<u64>().unwrap() * self.d[i].items[0];
                            }
                        },
                        "+" => {
                            if self.d[i].op.0.as_str() == "old" && self.d[i].op.1.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].items[0] + self.d[i].items[0];
                            } else if self.d[i].op.0.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].op.1.parse::<u64>().unwrap() + self.d[i].items[0];
                            } else if self.d[i].op.1.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].op.0.parse::<u64>().unwrap() + self.d[i].items[0];
                            }
                        },
                        _ => panic!()
                    }
                    
                    let it = self.d[i].items.remove(0) / 3;
                    if it % self.d[i].test == 0 {
                        let idx: usize = self.d[i].throw.0;
                        self.d[idx].items.push(it);
                    } else {
                        let idx: usize = self.d[i].throw.1;
                        self.d[idx].items.push(it);
                    }
                    self.d[i].times += 1;
                }
            }
        }
        let mut max: Vec<u32> = Vec::new();

        for i in 0..self.d.len() {
            max.push(self.d[i].times);
        }

        max.sort();
        crate::output(max[max.len()-1]*max[max.len()-2])
    }
        
    fn part2(&mut self) -> Vec<String> {
        self.d.clear();
        self.parse();
        for _ in 0..20 {
            for i in 0..self.d.len() {
                while self.d[i].items.len() > 0 {
                    match self.d[i].kind.as_str() {
                        "*" => {
                            if self.d[i].op.0.as_str() == "old" && self.d[i].op.1.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].items[0] * self.d[i].items[0];
                            } else if self.d[i].op.0.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].op.1.parse::<u64>().unwrap() * self.d[i].items[0];
                            } else if self.d[i].op.1.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].op.0.parse::<u64>().unwrap() * self.d[i].items[0];
                            }
                        },
                        "+" => {
                            if self.d[i].op.0.as_str() == "old" && self.d[i].op.1.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].items[0] + self.d[i].items[0];
                            } else if self.d[i].op.0.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].op.1.parse::<u64>().unwrap() + self.d[i].items[0];
                            } else if self.d[i].op.1.as_str() == "old" {
                                self.d[i].items[0] = self.d[i].op.0.parse::<u64>().unwrap() + self.d[i].items[0];
                            }
                        },
                        _ => panic!()
                    }
                    
                    let it = self.d[i].items.remove(0);

                    if it % self.d[i].test == 0 {
                        let idx: usize = self.d[i].throw.0;
                        self.d[idx].items.push(it);
                    } else {
                        let idx: usize = self.d[i].throw.1;
                        self.d[idx].items.push(it);
                    }
                    self.d[i].times += 1;
                }
            }
        }
        let mut max: Vec<u32> = Vec::new();

        for i in 0..self.d.len() {
            max.push(self.d[i].times);
        }
        println!("{:?}", max);

        max.sort();
        crate::output(max[max.len()-1]*max[max.len()-2])
    }
}