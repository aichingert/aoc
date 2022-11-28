pub struct Aoc2016_05 {
    s: String
}
        
impl Aoc2016_05 {
    pub fn new() -> Self {
        Self { s: String::new() }
    }

    fn get_hash(s: &str) -> String {
        format!("{:X}", md5::compute(s.as_bytes()))
    }
}
        
impl crate::Solution for Aoc2016_05 {
    fn name(&self) -> (usize, usize) {
        (2016, 05)
    }
        
    fn parse(&mut self) {
        self.s = std::fs::read_to_string("input/2016/05.txt")
            .expect("unable to open file")
            .trim()
            .to_string(); 
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut i: usize = 0;
        let mut password: String = String::new();
        while password.len() < 8 {
            let s: Vec<char> = Aoc2016_05::get_hash(&(self.s.clone() + &i.to_string())).chars().collect();
            if s[0] == '0' && s[1] == '0' && s[2] == '0' && s[3] == '0' && s[4] == '0' {
                password.push(s[5]);
            }

            i+=1;
        }

        crate::output(password)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut i: usize = 0;
        let mut password: Vec<char> = vec![' '; 8];
        let mut is_done: bool = false;

        while !is_done {
            let s: Vec<char> = Aoc2016_05::get_hash(&(self.s.clone() + &i.to_string())).chars().collect();
            if s[0] == '0' && s[1] == '0' && s[2] == '0' && s[3] == '0' && s[4] == '0' 
            && s[5].is_digit(10) && (s[5] as u8 - '0' as u8) < 8u8 && password[(s[5] as u8 - '0' as u8) as usize] == ' ' {
                password[(s[5] as u8 - '0' as u8) as usize] = s[6];

                if password.iter().filter(|c| **c != ' ').count() == 8 {
                    is_done = true;
                }
            }

            i+=1;
        }

        crate::output(password.iter().collect::<String>())
    }
}
