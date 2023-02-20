// Advent of Code 2019, Intcode
// (c) aichingert

pub struct Computer {
    pub opcodes: Vec<i32>,
    input: Vec<i32>,
    pub output: Vec<i32>,
}

impl Computer {
    pub fn new(opcodes: Vec<i32>, input: Vec<i32>) -> Self {
        Self { opcodes, input, output: Vec::<i32>::new() }
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

    fn get_jump(&self, index: &mut usize, mode: &Vec<usize>) -> (usize,i32) {
        let mut locations = Vec::<usize>::new();

        for i in 2..4 {
            match mode[3 - i] {
                0 => locations.push(self.opcodes[*index+i-1] as usize),
                1 => locations.push(*index+i-1),
                _ => panic!("Invalid mode: {}", mode[i]),
            }
        }

        *index += 3;
        (self.opcodes[locations[1]] as usize,self.opcodes[locations[0]])
    }

    pub fn run(&mut self) {
        let mut index = 0usize;
        let mut input = 0usize;

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
                    self.opcodes[at] = self.input[input];
                    input += 1;
                },
                4 => {
                    let at = self.get_io(&mut index,&commands);
                    self.output.push(self.opcodes[at]);
                },
                5 => {
                    let (at,val) = self.get_jump(&mut index,&commands);
                    if val != 0 { index = at; }
                },
                6 => {
                    let (at,val) = self.get_jump(&mut index,&commands);
                    if val == 0 { index = at; }
                },
                7 => {
                    let (at,a,b) = self.get_vals(&mut index, &commands);
                    match a < b { 
                        true => self.opcodes[at] = 1,
                        false => self.opcodes[at] = 0,
                    };
                },
                8 => {
                    let (at,a,b) = self.get_vals(&mut index, &commands);
                    match a == b { 
                        true => self.opcodes[at] = 1,
                        false => self.opcodes[at] = 0,
                    };
                },
                99 => break,
                _ => panic!("Invalid opcode {}", opcode)
            }
        }
    }
}
