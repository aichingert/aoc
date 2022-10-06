pub struct Aoc2020_18 {
    line: Vec<Vec<String>>
}
        
impl Aoc2020_18 {
    pub fn new() -> Self {
        Self { line: vec![] }
    }

    fn calculate(line: &Vec<String>) -> u64 {
        let mut idx: usize = 0;

        if line.len() == 1 {
            return line[0].parse::<u64>().unwrap();
        }

        let mut result: u64 = match &line[idx+1] as &str {
            "+" => {
                line[idx].parse::<u64>().unwrap()+line[idx+2].parse::<u64>().unwrap()
            },
            "*" => {
                line[idx].parse::<u64>().unwrap()*line[idx+2].parse::<u64>().unwrap()
            },
            _ => panic!("invalid operator")
        };
 
        idx=3;
        while idx < line.len() {
            match &line[idx] as &str {
                "+" => {
                    result+=line[idx+1].parse::<u64>().unwrap();
                },
                "*" => {
                    result*=line[idx+1].parse::<u64>().unwrap();
                },
                _ => panic!("invalid operator")
            }
            idx+=2;
        }

        result
    }

    fn unzip(values: &mut Vec<String>, idx: usize) -> u64 {
        let mut equation: Vec<String> = vec![];

        if &values[idx][0..=0] == "(" {
            values[idx] = values[idx][1..values[idx].len()].to_string();
            equation.push(Aoc2020_18::unzip(values, idx).to_string());
        } 

        while idx < values.len() {
            if &values[idx][0..=0] == "(" {
                values[idx] = values[idx][1..values[idx].len()].to_string();
                equation.push(Aoc2020_18::unzip(values, idx).to_string());
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

        Aoc2020_18::calculate(&equation)
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
        crate::output(self.line.iter_mut().map(|l| Aoc2020_18::unzip(l, 0)).sum::<u64>())
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
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
        assert!(Aoc2020_18::unzip(&mut (String::from("2 * 3 + (4 * 5)").split(' ').map(|s| s.to_string()).collect()), 0) == 26);
        assert!(Aoc2020_18::unzip(&mut (String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)").split(' ').map(|s| s.to_string()).collect()), 0) == 437);
        assert!(Aoc2020_18::unzip(&mut (String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").split(' ').map(|s| s.to_string()).collect()), 0) == 12240);
        assert!(Aoc2020_18::unzip(&mut (String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").split(' ').map(|s| s.to_string()).collect()), 0) == 13632);
        assert!(Aoc2020_18::unzip(&mut (String::from("7 + 3 * 9 + (2 * (2 + 9 + 9 + 8 + 9) * (8 * 6)) * 4 * 9").split(' ').map(|s| s.to_string()).collect()), 0) == 131112);
    }
}