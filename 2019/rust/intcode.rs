// Advent of Code 2019, Intcode
// (c) aichingert

pub struct Computer {
    pub opcodes: Vec<usize>,
}

impl Computer {
    pub fn new(opcodes: Vec<usize>) -> Self {
        Self { opcodes }
    }

    fn get_pos(&self, index: usize) -> (usize,usize,usize) {
        (self.opcodes[index+3],self.opcodes[index+1],self.opcodes[index+2])
    }

    pub fn run(&mut self) {
        let mut index = 0usize;

        while index < self.opcodes.len() {
            match self.opcodes[index] {
                1 => {
                    let (ins, fst, scn) = self.get_pos(index);
                    self.opcodes[ins] = self.opcodes[fst] + self.opcodes[scn];
                },
                2 => {
                    let (ins, fst, scn) = self.get_pos(index);
                    self.opcodes[ins] = self.opcodes[fst] * self.opcodes[scn];
                },
                _ => { break; }
            }
            
            index += 4;
        }
    }
}
