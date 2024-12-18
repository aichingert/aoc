fn combo(rgs: &[u64], op: u64) -> u64 {
    assert!(op < 7);

    if op <= 3 {
        op
    } else {
        rgs[op as usize - 4]
    }
}

fn find(prg: &[u64], pos: usize, mut a: u64) -> u64 {
    if &sol(&[a,0,0], prg) == prg {
        return a;
    }
    if pos >= prg.len() {
        return u64::MAX;
    }

    let mut ans = u64::MAX;

    for s in 0..8 {
        let out = sol(&[a + s, 0, 0], prg);

        if prg[prg.len() - 1 - pos] == out[0] {
            if &out == prg {
                ans = ans.min((a + s));
            }
            ans = ans.min(find(prg, pos + 1, (a + s) * 8));
        }
    }

    ans
}

fn sol(rgs: &[u64], prg: &[u64]) -> Vec<u64> {
    let mut pc = 0;
    let mut out = Vec::new();
    let mut rgs = [rgs[0], rgs[1], rgs[2]];

    while pc < prg.len() {
        match prg[pc] {
            0 => rgs[0] >>= combo(&rgs, prg[pc + 1]),
            1 => rgs[1] ^= prg[pc + 1],
            2 => rgs[1] = combo(&rgs, prg[pc + 1]) % 8,
            3 => if rgs[0] != 0 {
                pc = prg[pc + 1] as usize;
                continue;
            }
            4 => rgs[1] ^= rgs[2],
            5 => out.push(combo(&rgs, prg[pc + 1]) % 8),
            6 => rgs[1] = rgs[0] >> combo(&rgs, prg[pc + 1]),
            7 => rgs[2] = rgs[0] >> combo(&rgs, prg[pc + 1]), 
            _ => unreachable!(),
        }

        pc += 2;
    }

    out
}

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/17").unwrap();
    let (regs, program) = inp.split_once("\n\n").unwrap();

    let mut rgs = regs.lines().map(|n| n.split(": ").skip(1).next().unwrap().parse::<u64>().unwrap()).collect::<Vec<_>>();
    let prg = program.split_once(": ").unwrap().1.split(",").map(|n| n.trim().parse::<u64>().unwrap()).collect::<Vec<_>>();

    println!("p1: {}", sol(&rgs, &prg).iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","));
    println!("p2: {}", find(&prg, 0, 0));
}
