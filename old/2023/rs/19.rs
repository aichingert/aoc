use std::collections::HashMap;

fn part_one(ratings: &Vec<u64>, workflows: &HashMap<String, Workflow>, current: &String) -> u64 {
    match current.as_str() {
        "A" => return ratings.iter().sum::<u64>(),
        "R" => return 0,
        _ => (),
    };

    let workflow = &workflows[current];

    for i in 0..workflow.rules.len() {
        if workflow.rules[i].applies(ratings) {
            return part_one(ratings, workflows, &workflow.rules[i].mapping_to);
        }
    }

    part_one(ratings, workflows, &workflow.default)
}

fn part_two(mut range: Vec<(u64, u64)>, workflows: &HashMap<String, Workflow>, current: &String) -> u64 {
    match current.as_str() {
        "A" => return range.iter().map(|n| (1 + n.1 - n.0)).fold(1, |acc,cur| acc * cur),
        "R" => return 0,
        _ => (),
    };

    let workflow = &workflows[current];
    let mut total = 0u64;

    for i in 0..workflow.rules.len() {
        if let Some(modified) = workflow.rules[i].split_range(&mut range) {
            total += part_two(modified, workflows, &workflow.rules[i].mapping_to);
        }
    }

    total + part_two(range, workflows, &workflow.default)
}

fn idx(s: &str) -> usize {
    match s {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("invalid"),
    }
}

struct Workflow {
    rules: Vec<Rule>,
    default: String,
}

struct Rule {
    idx: usize,
    operator: Operator,
    amount: u64,
    mapping_to: String,
}

impl Rule {
    fn applies(&self, ratings: &Vec<u64>) -> bool {
        match self.operator {
            Operator::Smaller => ratings[self.idx] < self.amount,
            Operator::Greater => ratings[self.idx] > self.amount,
        }
    }

    fn split_range(&self, range: &mut Vec<(u64, u64)>) -> Option<Vec<(u64, u64)>> {
        Some(match self.operator {
            Operator::Smaller => {
                if range[self.idx].0 >= self.amount {
                    return None;
                } 

                let mut clone = range.clone();
                range[self.idx].0 = self.amount;
                clone[self.idx].1 = self.amount - 1;

                clone
            }
            Operator::Greater => {
                if range[self.idx].1 <= self.amount {
                    return None;
                }

                let mut clone = range.clone();
                range[self.idx].1 = self.amount;
                clone[self.idx].0 = self.amount;

                clone
            }
        })
    }
}

enum Operator {
    Smaller,
    Greater,
}

fn parse() -> (String, HashMap<String, Workflow>, String) {
    let inp = std::fs::read_to_string("../input/19").unwrap().trim().to_string();
    let inp = inp.split("\n\n").map(|s| s).collect::<Vec<_>>();

    let mut workflows = HashMap::new();
    let start = String::from("in");

    for line in inp[0].lines() {
        let (name, rest) = line.split_once('{').unwrap();
        let conditions = rest[..rest.len() - 1].split(',').collect::<Vec<_>>();
        let mut rules = Vec::new();

        for i in 0..conditions.len() - 1 {
            let (condition, mapping_to) = conditions[i].split_once(":").unwrap();

            let (idx, operator, amount) = if let Some((ident, amount)) = condition.split_once("<") {
                (idx(ident), Operator::Smaller, amount.parse::<u64>().unwrap())
            } else {
                let (ident, amount) = condition.split_once(">").unwrap();
                (idx(ident), Operator::Greater, amount.parse::<u64>().unwrap())
            };

            rules.push(Rule { idx, operator, amount, mapping_to: mapping_to.to_string() });
        }

        workflows.insert(name.to_string(), Workflow {
            rules,
            default: conditions[conditions.len() - 1].to_string()
        });
    }

    (start, workflows, inp[1].to_string())
}

fn main() {
    let (start, workflows, test) = parse();

    println!("Part one: {}", test.lines()
        .map(|l| part_one(&l[1..l.len() - 1].split(',').map(|s| s[2..].parse().unwrap()).collect(), 
                    &workflows, 
                    &start))
        .sum::<u64>());
    println!("Part two: {}", part_two(vec![(1,4000),(1,4000),(1,4000),(1,4000)], &workflows, &start));
}
