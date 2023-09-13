const PATTERN: [i16;2] = [1, -1];

fn phase(inp: Vec<u16>) -> Vec<u16> {
    let mut output = Vec::new();

    for i in 0..inp.len() {
        let mut ptr = 0;
        let mut out = 0;

        let mut j = i;

        while j < inp.len() {
            let mut s = 0;
            while s <= i && j < inp.len() {
                out += (inp[j] as i16) * PATTERN[ptr];

                s += 1;
                j += 1;
            }

            j += i + 1;
            ptr = 1 - ptr;
        }

        output.push((out.abs() as u16) % 10);
    }

    output
}

fn part_two(mut inp: Vec<u16>, offset: usize) -> Vec<u16> {
    for _ in 0..100 {
        for i in (offset..inp.len() - 1).rev() {
            inp[i] = (inp[i] + inp[i + 1]) % 10;
        }
    }

    inp
}

fn to_string(number: &[u16]) -> String {
    number.iter().map(|c| (*c as u8 + b'0') as char).collect::<String>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/16").unwrap().trim().to_string();
    let mut part_one = inp.chars().map(|c| (c as u8 - b'0').into()).collect::<Vec<u16>>();
    let mut output = Vec::new();

    for _ in 0..10000 { output.extend_from_slice(&part_one.clone()); }
    let offset = inp[..7].parse::<usize>().unwrap();

    for _ in 0..100 {
        part_one = phase(part_one);
    }

    output = part_two(output, offset);

    println!("Part one: {}", to_string(&part_one[..8]));
    println!("Part two: {}", to_string(&output[offset..offset+ 8]));
}
