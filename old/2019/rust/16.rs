const PATTERN: [i16;4] = [0, 1, 0, -1];

fn phase(inp: Vec<u16>) -> Vec<u16> {
    let mut output = Vec::new();

    for i in 0..inp.len() {
        let i = i;
        let mut mathi = 0;
        let mut ptr = 1;
        let mut out = 0;

        for j in i..inp.len() {
            out += (inp[j] as i16) * PATTERN[ptr];

            mathi += 1;
            if mathi > i {
                ptr = (ptr + 1) % PATTERN.len();
                mathi = 0;
            }
        }

        output.push((out.abs() as u16) % 10);
    }

    output
}

fn main() {
    let inp = std::fs::read_to_string("../input/16").unwrap().trim().to_string();
    let mut output = inp.chars().map(|c| (c as u8 - b'0').into()).collect::<Vec<u16>>();

    for _ in 0..4 {
        output = phase(output);
    }

    println!("{:?}", &output);
}
