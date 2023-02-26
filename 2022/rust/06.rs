// Advent of Code 2022, day 6
// (c) aichingert

fn solve(inp: &[u8],len:usize) -> usize {
    let mut idx: usize = 0;

    while let Some(arr) = inp.get(idx..idx + len) {
        let mut bits: u32 = 0;

        if let Some(pos) = arr.iter().rposition(|byte| {
            let bit_loc = byte % 32;
            let ret = bits & (1 << bit_loc) != 0;
            bits |= 1 << bit_loc;
            ret
        }) {
            idx += pos + 1;
        } else {
            return idx + len;
        }
    }

    0
}

fn main() {
    let inp = std::fs::read_to_string("../input/06").unwrap().bytes().collect::<Vec<u8>>();

    println!("Part 1: {}", solve(&inp,4));
    println!("Part 2: {}", solve(&inp,14));
}
