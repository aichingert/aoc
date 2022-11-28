pub struct Aoc2020_18 {
    line: Vec<Vec<String>>
}
        
impl Aoc2020_18 {
    pub fn new() -> Self {
        Self { line: vec![] }
    }

    fn calculate(line: &Vec<String>) -> usize {
        if line.len() == 1 {
            return line[0].parse::<usize>().unwrap();
        }

        let mut idx: usize = 0;
        let mut result: usize = match &line[idx+1] as &str {
            "+" => line[idx].parse::<usize>().unwrap()+line[idx+2].parse::<usize>().unwrap(),
            "*" => line[idx].parse::<usize>().unwrap()*line[idx+2].parse::<usize>().unwrap(),
            _ => panic!("invalid operator")
        };
 
        idx=3;
        while idx < line.len() {
            match &line[idx] as &str {
                "+" => result+=line[idx+1].parse::<usize>().unwrap(),
                "*" => result*=line[idx+1].parse::<usize>().unwrap(),
                _ => panic!("invalid operator")
            }
            idx+=2;
        }

        result
    }

    fn calculate_with_priority(line: &Vec<String>) -> usize {
        if line.len() == 1 {
            return line[0].parse::<usize>().unwrap();
        }

        let mut idx: usize = 3;
        let mut priority_eq: Vec<String> = match &line[1] as &str {
            "+" => vec![(line[0].parse::<usize>().unwrap() + line[2].parse::<usize>().unwrap()).to_string()],
            "*" => vec![line[0].to_string(), line[1].to_string(), line[2].to_string()],
            _ => panic!()
        };

        while idx < line.len() {
            let len: usize = priority_eq.len()-1;
            match &line[idx] as &str {
                "+" => priority_eq[len] = (priority_eq[len].parse::<usize>().unwrap() + line[idx+1].parse::<usize>().unwrap()).to_string(),
                "*" => {
                    priority_eq.push(line[idx].to_string());
                    priority_eq.push(line[idx+1].to_string());
                }
                _ => panic!("invalid operator!")
            }
            idx+=2;
        }
        
        Aoc2020_18::calculate(&priority_eq)
    }

    fn unzip(values: &mut Vec<String>, idx: usize, part: bool) -> usize {
        let mut equation: Vec<String> = vec![];

        if &values[idx][0..=0] == "(" {
            values[idx] = values[idx][1..values[idx].len()].to_string();
            equation.push(Aoc2020_18::unzip(values, idx, part).to_string());
        } 

        while idx < values.len() {
            if &values[idx][0..=0] == "(" {
                values[idx] = values[idx][1..values[idx].len()].to_string();
                equation.push(Aoc2020_18::unzip(values, idx, part).to_string());
            } else {
                if values[idx].len() >= 2 {
                    equation.push(values[idx][0..=0].to_string());
                    if values[idx].len() > 2 {
                        values[idx] = values[idx][2..values[idx].len()].to_string();
                    } else {
                        values.remove(idx);
                    }
                    break;
                } else if values[idx] == ")" {
                    values.remove(idx);
                    break;
                }
                equation.push(values.remove(idx));
            }
        }

        if part {
            Aoc2020_18::calculate(&equation)
        } else {
            Aoc2020_18::calculate_with_priority(&equation)
        }
    }
}
        
impl crate::Solution for Aoc2020_18 {
    fn name(&self) -> (usize, usize) {
        (2020, 18)
    }
        
    fn parse(&mut self) {
        self.line = aoc::read_to_slice("input/2020/18.txt", " ");
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output(self.line.iter_mut().map(|l| Aoc2020_18::unzip(l, 0, true)).sum::<usize>())
    }
        
    fn part2(&mut self) -> Vec<String> {
        self.parse();
        crate::output(self.line.iter_mut().map(|l| Aoc2020_18::unzip(l, 0, false)).sum::<usize>())
    }
}

#[cfg(test)]
mod test {
    use crate::aoc2020::aoc2020_18::*;

    #[test]
    fn aoc2020_18_calculate_function() {
        assert!(Aoc2020_18::calculate(&(String::from("1 + 2 * 3 + 4 * 5 + 6").split(' ').map(|s| s.to_string()).collect())) == 71);
        assert!(Aoc2020_18::calculate(&(String::from("4 * 8 + 2").split(' ').map(|s| s.to_string()).collect())) == 34);
    }

    #[test]
    fn aoc2020_18_unzip_function() {
        assert!(Aoc2020_18::unzip(&mut (String::from("2 * 3 + (4 * 5)").split(' ').map(|s| s.to_string()).collect()), 0, true) == 26);
        assert!(Aoc2020_18::unzip(&mut (String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)").split(' ').map(|s| s.to_string()).collect()),0, true) == 437);
        assert!(Aoc2020_18::unzip(&mut (String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").split(' ').map(|s| s.to_string()).collect()),0, true) == 12240);
        assert!(Aoc2020_18::unzip(&mut (String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").split(' ').map(|s| s.to_string()).collect()),0, true) == 13632);
        assert!(Aoc2020_18::unzip(&mut (String::from("7 + 3 * 9 + (2 * (2 + 9 + 9 + 8 + 9) * (8 * 6)) * 4 * 9").split(' ').map(|s| s.to_string()).collect()),0, true) == 131112);
    }

    
    #[test]
    fn aoc2020_18_calculate_with_priority_fn() {
        assert!(Aoc2020_18::calculate_with_priority(&(String::from("1 + 2 * 3 + 4 * 5 + 6").split(' ').map(|s| s.to_string()).collect())) == 231);
        assert!(Aoc2020_18::unzip(&mut (String::from("1 + (2 * 3) + (4 * (5 + 6))").split(' ').map(|s| s.to_string()).collect()),0, false) == 51);
        assert!(Aoc2020_18::unzip(&mut (String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").split(' ').map(|s| s.to_string()).collect()),0, false) == 669060);
        assert!(Aoc2020_18::unzip(&mut (String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").split(' ').map(|s| s.to_string()).collect()),0, false) == 23340);
    }
}
