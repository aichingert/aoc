// Advent of Code 2018, day 19
// (c) aichingert

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Opcode {
    fn from_str(s: &str) -> Self {
        match s {
            "addr" => Self::Addr,
            "addi" => Self::Addi,
            "mulr" => Self::Mulr,
            "muli" => Self::Muli,
            "banr" => Self::Banr,
            "bani" => Self::Bani,
            "borr" => Self::Borr,
            "bori" => Self::Bori,
            "setr" => Self::Setr,
            "seti" => Self::Seti,
            "gtir" => Self::Gtir,
            "gtri" => Self::Gtri,
            "gtrr" => Self::Gtrr,
            "eqir" => Self::Eqir,
            "eqri" => Self::Eqri,
            "eqrr" => Self::Eqrr,
            _ => panic!("invalid opcode {}", s),
        }
    }

    fn execute(&self, reg: &mut [i64; 6], n: &Vec<i64>) {
        reg[n[2] as usize] = match self {
            Opcode::Addr => reg[n[0] as usize] + reg[n[1] as usize],
            Opcode::Addi => reg[n[0] as usize] + n[1],
            Opcode::Seti => n[0],
            Opcode::Setr => reg[n[0] as usize],
            Opcode::Mulr => reg[n[0] as usize] * reg[n[1] as usize],
            Opcode::Muli => reg[n[0] as usize] * n[1],
            Opcode::Banr => reg[n[0] as usize] & reg[n[1] as usize],
            Opcode::Bani => reg[n[0] as usize] & n[1],
            Opcode::Borr => reg[n[0] as usize] | reg[n[1] as usize],
            Opcode::Bori => reg[n[0] as usize] | n[1],
            Opcode::Gtir => {
                if n[0] > reg[n[1] as usize] {
                    1
                } else {
                    0
                }
            }
            Opcode::Gtri => {
                if reg[n[0] as usize] > n[1] {
                    1
                } else {
                    0
                }
            }
            Opcode::Gtrr => {
                if reg[n[0] as usize] > reg[n[1] as usize] {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqir => {
                if n[0] == reg[n[1] as usize] {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqri => {
                if reg[n[0] as usize] == n[1] {
                    1
                } else {
                    0
                }
            }
            Opcode::Eqrr => {
                if reg[n[0] as usize] == reg[n[1] as usize] {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn solve(start: i64, bound: usize, instr: &Vec<(Opcode, Vec<i64>)>) -> i64 {
    let mut reg = [0i64; 6];
    reg[0] = start;
    let mut ip: usize = 0;

    while ip < instr.len() {
        if ip == 3 {
            if reg[2] % reg[1] == 0 {
                reg[0] = reg[1] + reg[0];
            }

            reg[4] = reg[2];
            reg[5] = 0;
            ip = 12;
            continue;
        }

        reg[bound] = ip as i64;
        instr[ip].0.execute(&mut reg, &instr[ip].1);
        ip = reg[bound] as usize + 1;
    }

    reg[0]
}

fn parse() -> (usize, Vec<(Opcode, Vec<i64>)>) {
    let inp = std::fs::read_to_string("../input/19").unwrap();
    let inp = inp.lines().collect::<Vec<&str>>();

    let mut instr = Vec::<(Opcode, Vec<i64>)>::new();
    let bound = inp[0][4..5].parse::<usize>().unwrap();

    for i in 1..inp.len() {
        let vals = inp[i].split(' ').collect::<Vec<&str>>();
        instr.push((
            Opcode::from_str(vals[0]),
            vals[1..]
                .iter()
                .map(|i| i.parse().unwrap())
                .collect::<Vec<i64>>(),
        ));
    }

    (bound, instr)
}

fn main() {
    let (bound, instr) = parse();

    println!("Part 1: {}", solve(0, bound, &instr));
    println!("Part 2: {}", solve(1, bound, &instr));
}
