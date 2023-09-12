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

fn main() {
    let inp = std::fs::read_to_string("../input/16").unwrap().trim().to_string();
    let mut output = inp.chars().map(|c| (c as u8 - b'0').into()).collect::<Vec<u16>>();

    for _ in 0..100 {
        output = phase(output);
    }

    println!("{:?}", &output[0..8]);
}
