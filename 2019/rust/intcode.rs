// Advent of Code 2019, Intcode
// (c) aichingert

pub struct Computer {
    pub opcodes: Vec<i32>,
    input: i32,
}

impl Computer {
    pub fn new(opcodes: Vec<i32>, input: i32) -> Self {
        Self { opcodes, input }
    }

    fn get_vals(&self, index: &mut usize, mode: &Vec<usize>) -> (usize,i32,i32) {
        let mut locations = Vec::<usize>::new();

        for i in 2..4 {
            match mode[3 - i] {
                0 => locations.push(self.opcodes[*index+i-1] as usize),
                1 => locations.push(*index+i-1),
                _ => panic!("Inavlid mode: {}", mode[i]),
            }
        }

        *index += 4;
        (self.opcodes[*index-1] as usize,self.opcodes[locations[0]],self.opcodes[locations[1]])
    }

    fn get_io(&self, index: &mut usize, mode: &Vec<usize>) -> usize {
        *index += 2;
        
        match mode[1] {
            0 => self.opcodes[*index-1] as usize,
            1 => *index-1,
            _ => panic!("Invalid mode: {}", mode[1]),
        }
    }

    pub fn run(&mut self) {
        let mut index = 0usize;

        while index < self.opcodes.len() {
            let mut commands = self.opcodes[index].to_string().chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();
            while commands.len() < 4 { commands.insert(0, 0); }
            let opcode = commands[2] * 10 + commands[3];
                
            match opcode {
                1 => {
                    let (at,a,b) = self.get_vals(&mut index, &commands);
                    self.opcodes[at] = a + b;
                },
                2 => {
                    let (at,a,b) = self.get_vals(&mut index, &commands);
                    self.opcodes[at] = a * b;
                },
                3 => {
                    let at = self.get_io(&mut index,&commands);
                    self.opcodes[at] = self.input;
                },
                4 => {
                    let at = self.get_io(&mut index,&commands);
                    println!("{:?}", self.opcodes[at]);
                },
                99 => { break; }
                _ => panic!("Invalid opcode {}", opcode)
            }
        }
    }
}
