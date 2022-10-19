use aoc::numbers;

pub struct Aoc2021_16 {
    d: Vec<Vec<usize>>
}

impl Aoc2021_16 {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    fn unwrap_packet(&self, bits: &[u8]) -> Vec<i32> {

        vec![self.version(bits), self.id(bits), self.get_group(bits)]
    }

    fn version(&self, bits: &[u8]) -> i32 {
        let mut version: i32 = 0;

        for i in 0..3 {
            version += bits[2 - i] as i32 * 2_i32.pow(i as u32);
        }

        version
    }

    fn id(&self, bits: &[u8]) -> i32 {
        let mut id: i32 = 0;

        for i in 3..6 {
            id += bits[8-i] as i32 * 2_i32.pow((i-3) as u32);
        }

        id
    }

    fn get_group(&self, bits: &[u8]) -> i32 {
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

    fn calc_bin_to_dec(&self, bits: &[u8]) -> i32 {
        bits.iter().enumerate().map(|(i,e)|
            *e as i32 * 2_i32.pow((bits.len() - i - 1) as u32)
        ).sum::<i32>()
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
