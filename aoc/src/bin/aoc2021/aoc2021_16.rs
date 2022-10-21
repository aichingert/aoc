use aoc::numbers;

pub struct Aoc2021_16 {
    d: Vec<Vec<usize>>
}

#[derive(Debug)]
struct Packet {
    version: u64,
    id: u64,
    value: u64
}

impl Packet {
    fn new(version: u64, id: u64, value: u64) -> Self {
        Self {
            version,
            id,
            value
        }
    }
}

#[derive(Debug)]
struct Operator {
    version: u64,
    id: u64,
    packets: Vec<Packet>
}

impl Operator {
    fn new(version: u64, id: u64) -> Self {
        Self {
            version,
            id,
            packets: Vec::new()
        }
    }
}

impl Aoc2021_16 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn unwrap_packet(&self, bits: &[u8]) -> Packet {
        let version = self.version(bits);
        let mut literal: u64 = 0;

        match version {
            4 => literal = self.get_literal(bits),
            _ => {

            }
        }

        Packet::new(version, self.id(bits), self.get_literal(bits))
    }

    fn version(&self, bits: &[u8]) -> u64 {
       self.calc_bin_to_dec(&bits[0..3])
    }

    fn id(&self, bits: &[u8]) -> u64 {        
        self.calc_bin_to_dec(&bits[3..6])
    }

    fn get_literal(&self, bits: &[u8]) -> u64 {
        let mut idx: usize = 6;
        let mut stop: bool = false;
        let mut to_calc: Vec<u8> = Vec::new();

        while !stop {
            if bits[idx] == 0 {
                stop = true;
            }

            for i in idx+1..idx+5 {
                to_calc.push(bits[i]);
            }

            idx+=5;
        }

        self.calc_bin_to_dec(&to_calc)
    }

    fn get_operator(&self, bits: &[u8], version: u64) -> Operator {
        let id: u64 = self.id(bits);
        let mut operator = Operator::new(version, id);
        let mode: bool = bits[6] == 0;
        let mut size: usize = 0;

        if mode {
            size = self.calc_bin_to_dec(&bits[7..22]) as usize;
        } else {
            size = self.calc_bin_to_dec(&bits[7..18]) as usize;


        }




        operator
    }

    fn calc_bin_to_dec(&self, bits: &[u8]) -> u64 {
        bits.iter().enumerate().map(|(i,e)|
            *e as u64 * 2_u64.pow((bits.len() - i - 1) as u32)
        ).sum::<u64>()
    }
}

impl crate::Solution for Aoc2021_16 {
    fn name(&self) -> (usize, usize) {
        (2021, 16)
    }

    fn parse(&mut self) {
        self.d = numbers("input/2021/16.txt", ",");

        println!("{:?}", self.unwrap_packet(&[1,1,0,1,0,0,1,0,1,1,1,1,1,1,1,0,0,0,1,0,1,0,0,0]));
    }

    fn part1(&mut self) -> Vec<String> {
        crate::output("")
    }

    fn part2(&mut self) -> Vec<String> {
       crate::output("")
    }
}
