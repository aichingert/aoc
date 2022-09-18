pub struct Aoc2015_10 {
    number: String
}
        
impl Aoc2015_10 {
    pub fn new() -> Self {
        Self {
            number: String::new()
        }
    }

    fn say(&self) -> String {
        let mut ans: String = String::new();
        let mut iter = self.number.chars().peekable();

        while let Some(cur) = iter.next() {
            let mut count: i32 = 1;
            while let Some(next) = iter.peek() {
                if *next == cur {
                    count += 1;
                    iter.next();
                } else {
                    break;
                }
            }
            
            ans.push_str(format!("{count}{cur}").as_str());
        } 

        ans
    }
}
        
impl crate::Solution for Aoc2015_10 {
    fn name(&self) -> (usize, usize) {
        (2015, 10)
    }
        
    fn parse(&mut self) {
        self.number = "1113222113".to_string();
    }
        
    fn part1(&mut self) -> Vec<String> {
        for _ in 0..40 {
            self.number = self.say();
        }
        crate::output(self.number.len())
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn say_number() {
        let mut day_10 = Aoc2015_10::new();

        day_10.number = "1".to_string();

        let mut next = day_10.say();

        assert!(next == "11");
        day_10.number = next;        
        next = day_10.say();

        assert!(next == "21");
        day_10.number = next;
        next = day_10.say();

        assert!(next == "1211");
        day_10.number = next;
        next = day_10.say();

        assert!(next == "111221");
        day_10.number = next;
    }
}