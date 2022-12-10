pub struct Aoc2022_10 {
    d: Vec<(String, i32)>
}
        
impl Aoc2022_10 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn p1(&self, c: i32, x: i32, p1: &mut i32) {
        if [20,60,100,140,180,220].contains(&c) {
            *p1 += c*x;
        }
    }

    fn p2(&self, c: &mut i32, x: i32, letters: &mut Vec<Vec<char>>) {
        letters[(*c/40) as usize][(*c%40) as usize] = if (x-(*c%40)).abs()<=1 { '#' } else { ' ' };
        *c += 1;
    }
}

impl crate::Solution for Aoc2022_10 {
    fn name(&self) -> (usize, usize) {
        (2022, 10)
    }
        
    fn parse(&mut self) {
        let input: Vec<Vec<String>> = aoc::read_to_slice("input/2022/10.txt", " ");

        for i in 0..input.len() {
            if input[i].len() > 1 {
                self.d.push((input[i][0].clone(), input[i][1].parse::<i32>().unwrap()));
            } else {
                self.d.push((input[i][0].clone(), 0));
            }
        }

    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut c: i32 = 0;
        let mut x: i32 = 1;
        let mut p1: i32 = 0;

        for i in 0..self.d.len() {
            match self.d[i].0.as_str() {
                "noop" => {
                    c+=1;
                    self.p1(c,x,&mut p1);
                },
                "addx" => {
                    c += 1;
                    self.p1(c,x,&mut p1);
                    c += 1;
                    self.p1(c,x,&mut p1);
                    x += self.d[i].1;
                },
                _ => panic!()
            }
        }
        crate::output(p1)
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut c: i32 = 0;
        let mut x: i32 = 1;
        let mut letters: Vec<Vec<char>> = vec![vec!['.';40];6]; 

        for i in 0..self.d.len() {
            match self.d[i].0.as_str() {
                "noop" => {               
                    self.p2(&mut c, x, &mut letters);   
                },
                "addx" => {
                    self.p2(&mut c, x, &mut letters); 
                    self.p2(&mut c, x, &mut letters);     
                    x += self.d[i].1;
                },
                _ => panic!()
            }
        }

        let mut o: String = String::new();
        for i in 0..letters.len() {
            o.push_str(&(format!("\n{}", letters[i].iter().collect::<String>())));
        }
        crate::output(o)
    }
}