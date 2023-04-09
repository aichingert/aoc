// Advent of Code 2017, day 10
// (c) aichingert

const SIZE: usize = 256;

fn hashing(list: &mut Vec<u32>, lengths: &Vec<usize>, skip_size: &mut usize, loc: &mut usize) -> u32 {
    for i in 0..lengths.len() {
        let mut col = (0..lengths[i]).map(|idx| list[(*loc + idx) % SIZE]).collect::<Vec<u32>>();
        col = col.iter().rev().map(|n| *n).collect::<Vec<u32>>();

        for j in 0..lengths[i] {
            list[(*loc + j) % SIZE] = col[j];
        }

        *loc = (*loc + lengths[i] + *skip_size) % SIZE;
        *skip_size += 1;
    }

    list[0] * list[1]
}

fn part2(hash: &str) -> String {
    let mut lengths = hash.as_bytes().iter().map(|n| *n as usize).collect::<Vec<usize>>();
    lengths.extend([17, 31, 73, 47, 23]);
    let (mut skip_size, mut loc) = (0usize,0usize);
    let mut sparse_hash = (0..SIZE as u32).collect::<Vec<u32>>();
    let mut dense_hash = vec![0;16];
    let mut knot_hash = String::new();

    for _ in 0..64 {
        hashing(&mut sparse_hash, &lengths, &mut skip_size, &mut loc);
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

fn main() {
    let inp = std::fs::read_to_string("../input/10").unwrap();
    let lengths: Vec<usize> = inp
        .split(',')
        .map(|n| n.trim().parse().unwrap())
        .collect();

    println!("Part 1: {}", hashing(&mut (0..SIZE as u32).collect::<Vec<u32>>(), &lengths, &mut 0, &mut 0));
    println!("Part 2: {}", part2(&inp.trim()));
}
