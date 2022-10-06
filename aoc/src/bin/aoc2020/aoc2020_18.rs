pub struct Aoc2020_18 {
    line: Vec<Vec<String>>
}
        
impl Aoc2020_18 {
    pub fn new() -> Self {
        Self { line: vec![] }
    }

    fn calculate(line: &Vec<String>) -> i32 {
        let mut idx: usize = 0;
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
}