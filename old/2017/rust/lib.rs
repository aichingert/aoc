// Advent of Code 2017 - lib
// (c) aichingert

pub const SIZE: usize = 256;

pub fn shuffling(list: &mut Vec<u32>, lengths: &Vec<usize>, skip_size: &mut usize, loc: &mut usize) -> u32 {
    for i in 0..lengths.len() {
        let mut src = *loc;
        let mut dst = (*loc + lengths[i] - 1) % SIZE;

        for _ in 0..lengths[i] / 2 {
            let tmp = list[src];
            list[src] = list[dst];
            list[dst] = tmp;

            src = (src + 1) % SIZE;
            if dst > 0 { dst -= 1; } else { dst = SIZE - 1; }
        }

        *loc = (*loc + lengths[i] + *skip_size) % SIZE;
        *skip_size += 1;
    }

    list[0] * list[1]
}

pub fn knot_hash(hash: &str) -> String {
    let mut lengths = hash.as_bytes().iter().map(|n| *n as usize).collect::<Vec<usize>>();
    lengths.extend([17, 31, 73, 47, 23]);
    let (mut skip_size, mut loc) = (0usize,0usize);
    let mut sparse_hash = (0..SIZE as u32).collect::<Vec<u32>>();
    let mut dense_hash = vec![0;16];
    let mut knot_hash = String::new();

    for _ in 0..64 {
        shuffling(&mut sparse_hash, &lengths, &mut skip_size, &mut loc);
    }

    for i in 0..16 {
        for j in 0..16 {
            dense_hash[i] ^= sparse_hash[i*16+j];
        }
    }

    for i in 0..dense_hash.len() {
        knot_hash.push_str(&format!("{:#04x}", dense_hash[i])[2..]);
    }

    knot_hash
}
