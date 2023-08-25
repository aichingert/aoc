#[derive(Clone)]
struct Operator {
    type_id: u16,
    packets: Vec<Packet>
}

#[derive(Clone)]
struct Literal {
    value: u64,
}

#[derive(Clone)]
enum Packet {
    Operator(Operator),
    Literal(Literal),
}

fn main() {
    let inp = std::fs::read_to_string("../input/16").unwrap();
    let stream = to_bin(inp);

    let mut part_one = 0;
    let packet = decode_package(&stream, &mut 0, &mut part_one);

    println!("Part 1: {}", part_one);
    println!("Part 2: {}", evaluate(packet));
}

fn evaluate(packet: Packet) -> u64 {
    match packet {
        Packet::Literal(literal) => {
            literal.value
        },
        Packet::Operator(operator) => {
            match operator.type_id {
                0 => operator.packets.into_iter().map(|p| evaluate(p)).sum::<u64>(),
                1 => operator.packets.into_iter().fold(1, |acc, p| acc * evaluate(p)),
                2 => operator.packets.into_iter().fold(u64::MAX, |acc, p| acc.min(evaluate(p))),
                3 => operator.packets.into_iter().fold(u64::MIN, |acc, p| acc.max(evaluate(p))),
                5 => if evaluate(operator.packets[0].clone()) > evaluate(operator.packets[1].clone()) { 
                    1 
                } else { 
                    0 
                }
                6 => if evaluate(operator.packets[0].clone()) < evaluate(operator.packets[1].clone()) { 
                    1 
                } else { 
                    0 
                }
                7 => if evaluate(operator.packets[0].clone()) == evaluate(operator.packets[1].clone()) { 
                    1 
                } else { 
                    0 
                }
                _ => panic!("invalid operator id {}", operator.type_id),
            }
        },
    }
}

fn decode_package(stream: &Vec<u16>, i: &mut usize, versions: &mut u32) -> Packet {
    let ptr = i;

    let version = stream[*ptr..*ptr + 3].iter().rev().enumerate()
        .map(|(p, bin)| bin * 2_u16.pow(p as u32))
        .sum::<u16>();
    *versions += version as u32;
    let id = stream[*ptr + 3..*ptr + 6].iter().rev().enumerate()
        .map(|(p, bin)| bin * 2_u16.pow(p as u32))
        .sum::<u16>();
    *ptr += 6;

    match id {
        4 => {
            let mut bits = Vec::<u16>::new();

            while stream[*ptr] != 0 {
                bits.extend_from_slice(&stream[*ptr+1..*ptr+5]);
                *ptr += 5;
            }
            bits.extend_from_slice(&stream[*ptr+1..*ptr+5]);
            *ptr += 5;

            Packet::Literal(Literal {
                value: bits.iter().rev().enumerate()
                    .map(|(p, bin)| (*bin as u64) * 2_u64.pow(p as u32))
                    .sum::<u64>()
            })
        },
        _ => {
            let mut operator = Operator {
                type_id: id,
                packets: Vec::new(),
            };

            match stream[*ptr] {
                0 => {
                    let len = stream[*ptr + 1..*ptr + 16]
                        .iter().rev().enumerate()
                        .map(|(p, bin)| (*bin as usize) * 2_usize.pow(p as u32)).sum::<usize>();
                    *ptr += 16;
                    let cur = *ptr;

                    while *ptr - cur < len {
                        operator.packets.push(decode_package(stream, ptr, versions));
                    }
                },
                1 => {
                    let n = stream[*ptr + 1..*ptr + 12]
                        .iter().rev().enumerate()
                        .map(|(p, bin)| (*bin as usize) * 2_usize.pow(p as u32)).sum::<usize>();
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
            _ => panic!("invalid hex char {c}"),
        };
    }

    binary
}
