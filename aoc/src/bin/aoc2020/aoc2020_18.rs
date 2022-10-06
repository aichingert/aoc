pub struct Aoc2020_18 {
    line: Vec<Vec<String>>
}
        
impl Aoc2020_18 {
    pub fn new() -> Self {
        Self { line: vec![] }
    }

    fn calculate(line: &Vec<String>) -> i32 {
        let mut idx: usize = 0;

        if line.len() == 1 {
            return line[0].parse::<i32>().unwrap();
        }

        let mut result: i32 = match &line[idx+1] as &str {
            "+" => {
                line[idx].parse::<i32>().unwrap()+line[idx+2].parse::<i32>().unwrap()
            },
            "*" => {
                line[idx].parse::<i32>().unwrap()*line[idx+2].parse::<i32>().unwrap()
            },
            _ => {
                0
            }
        };
 
        idx=3;
        while idx < line.len() {
            match &line[idx] as &str {
                "+" => {
                    result+=line[idx+1].parse::<i32>().unwrap();
                },
                "*" => {
                    result*=line[idx+1].parse::<i32>().unwrap();
                },
                _ => {}
            }
            idx+=1;
        }

        result
    }

    fn unzip(values: &mut Vec<String>, idx: usize) -> i32 {
        let mut equation: Vec<String> = vec![];

        if &values[idx][0..=0] == "(" {
            values[idx] = values[idx][1..values[idx].len()].to_string();
            equation.push(Aoc2020_18::unzip(values, idx).to_string());
        } 

        while idx < values.len() {
            if &values[idx][0..=0] == "(" {
                equation.push(Aoc2020_18::unzip(values, idx).to_string());
            } else {
                if values[idx].len() >= 2 {
                    equation.push(values[idx][0..=0].to_string());
                    if values[idx].len() > 2 {
                        values[idx] = values[idx][2..=values[idx].len()].to_string();
                    } else {
                        values.remove(idx);
                    }
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

        println!("{:?}", Aoc2020_18::calculate(&self.line[0]));
    }
        
    fn part1(&mut self) -> Vec<String> {
        crate::output("")
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
        assert!(Aoc2020_18::calculate(&vec![
            "1".to_string(), "+".to_string(), "2".to_string(), 
            "*".to_string(), "3".to_string(),"+".to_string(), 
            "4".to_string(), "*".to_string(), "5".to_string(), 
            "+".to_string(), "6".to_string()]) == 71
        );

        assert!(Aoc2020_18::calculate(&vec![
            "2".to_string(), "*".to_string(), "3".to_string()]) == 6
        );
    }

    #[test]
    fn aoc2020_18_unzip_function() {
        assert!(Aoc2020_18::unzip(&mut vec!["(2".to_string(), "+".to_string(), "3)".to_string()], 0) == 5);
    }
}