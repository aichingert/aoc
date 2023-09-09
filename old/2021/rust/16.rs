struct Operator {
    type_id: u16,
    packets: Vec<Packet>
}

enum Packet {
    Operator(Operator),
    Literal(u64),
}

fn main() {
    let stream = to_bin(std::fs::read_to_string("../input/16").unwrap());
    let mut part_one = 0;

    let packet = decode_package(&stream, &mut 0, &mut part_one);

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", evaluate(&packet));
}

fn evaluate(packet: &Packet) -> u64 {
    match packet {
        Packet::Literal(n) => *n,
        Packet::Operator(operator) => {
            let pckts = &operator.packets;
            match operator.type_id {
                0 => pckts.iter().map(|p| evaluate(p)).sum::<u64>(),
                1 => pckts.iter().fold(1, |acc, p| acc * evaluate(p)),
                2 => pckts.iter().fold(u64::MAX, |acc, p| acc.min(evaluate(p))),
                3 => pckts.iter().fold(u64::MIN, |acc, p| acc.max(evaluate(p))),
                5 => if evaluate(&pckts[0]) > evaluate(&pckts[1]) { 1 } else { 0 }
                6 => if evaluate(&pckts[0]) < evaluate(&pckts[1]) { 1 } else { 0 }
                7 => if evaluate(&pckts[0]) == evaluate(&pckts[1]) { 1 } else { 0 }
                _ => panic!("invalid operator id {}", operator.type_id),
            }
        },
    }
}

fn decode_package(stream: &Vec<u16>, ptr: &mut usize, versions: &mut u32) -> Packet {
    let version = to_dec(&stream[*ptr..*ptr + 3]) as u32;
    *versions += version as u32;

    let id = to_dec(&stream[*ptr + 3..*ptr + 6]) as u16;
    *ptr += 6;

    match id {
        4 => {
            let bits: usize = *ptr;

            while stream[*ptr] != 0 {
                *ptr += 5;
            }
            *ptr += 5;

            Packet::Literal(to_dec(&stream[bits..*ptr]))
        },
        _ => {
            let mut operator = Operator {
                type_id: id,
                packets: Vec::new(),
            };

            match stream[*ptr] {
                0 => {
                    let len = to_dec(&stream[*ptr + 1..*ptr + 16]) as usize;
                    *ptr += 16;
                    let cur = *ptr;

                    while *ptr - cur < len {
                        operator.packets.push(decode_package(stream, ptr, versions));
                    }
                },
                1 => {
                    let n = to_dec(&stream[*ptr + 1..*ptr + 12]);
                    *ptr += 12;

                    for _ in 0..n {
                        operator.packets.push(decode_package(stream, ptr, versions));
                    }
                },
                _ => panic!("invalid length type id {}", stream[*ptr]),
            }

            Packet::Operator(operator)
        }
    }
}

fn to_dec(s: &[u16]) -> u64 {
    let len = s.len();

    s.iter()
        .enumerate()
        .map(|(p, bin)| (*bin as u64) * 2_u64.pow((len as u32) - (p as u32) - 1))
        .sum::<u64>()
}

fn to_bin(s: String) -> Vec<u16> {
    let mut binary = Vec::new();

    for c in s.trim().chars() {
        match c {
            '0' => binary.extend_from_slice(&[0,0,0,0]),
            '1' => binary.extend_from_slice(&[0,0,0,1]),
            '2' => binary.extend_from_slice(&[0,0,1,0]),
            '3' => binary.extend_from_slice(&[0,0,1,1]),
            '4' => binary.extend_from_slice(&[0,1,0,0]),
            '5' => binary.extend_from_slice(&[0,1,0,1]),
            '6' => binary.extend_from_slice(&[0,1,1,0]),
            '7' => binary.extend_from_slice(&[0,1,1,1]),
            '8' => binary.extend_from_slice(&[1,0,0,0]),
            '9' => binary.extend_from_slice(&[1,0,0,1]),
            'A' => binary.extend_from_slice(&[1,0,1,0]),
            'B' => binary.extend_from_slice(&[1,0,1,1]),
            'C' => binary.extend_from_slice(&[1,1,0,0]),
            'D' => binary.extend_from_slice(&[1,1,0,1]),
            'E' => binary.extend_from_slice(&[1,1,1,0]),
            'F' => binary.extend_from_slice(&[1,1,1,1]),
            c => panic!("invalid hex char {}", c),
        };
    }

    binary
}
