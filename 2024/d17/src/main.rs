fn combo(rgs: &[u64], op: u64) -> u64 {
    assert!(op < 7);

    if op <= 3 {
        op
    } else {
        rgs[op as usize - 4]
    }
}

fn solution(rgs: &[u64], prg: &[u64]) -> Vec<u64> {
    let mut pc = 0;
    let mut out = Vec::new();
    let mut rgs = [rgs[0], rgs[1], rgs[2]];

    while pc < prg.len() {
        match prg[pc] {
            0 => {
                let v = 2_u64.pow(combo(&rgs, prg[pc + 1]) as u32);
                rgs[0] = rgs[0] / v;
            }
            1 => rgs[1] ^= prg[pc + 1],
            2 => rgs[1] = combo(&rgs, prg[pc + 1]) % 8,
            3 => if rgs[0] != 0 {
                pc = prg[pc + 1] as usize;
                continue;
            }
            4 => rgs[1] ^= rgs[2],
            5 => out.push(combo(&rgs, prg[pc + 1]) % 8),
            6 => {
                let v = 2_u64.pow(combo(&rgs, prg[pc + 1]) as u32);
                rgs[1] = rgs[0] / v;
            }
            7 => {
                let v = 2_u64.pow(combo(&rgs, prg[pc + 1]) as u32);
                rgs[2] = rgs[0] / v;
            }
            _ => unreachable!(),
        }

        pc += 2;
    }

    out
}

fn brute(prg: &[u64], hacked: &[u64]) -> u64 {
    'out: for &s in hacked {
        let mut a = s;
        let mut i = 0;
        let mut b;
        let mut c;

        while a != 0 {
            b = a % 8;
            b ^= 5;
            c = a >> b;
            b ^= c;
            b ^= 6;

            if b % 8 != prg[i] {
                continue 'out;
            }

            i += 1;
            a /= 8;
        }

        if i == prg.len() {
            return s;
        }
    }

    u64::MAX
}

fn find(len: usize, known: &[u64]) -> Vec<u64> {
    if len < known.len() + 1 {
        let mut ans = 0;

        for i in 0..known.len() {
            ans *= 8;
            ans += known[i];
        }

        return vec![ans];
    }

    let mut anss = Vec::new();

    for i in 0..8 {
        for ans in find(len - 1, known) {
            anss.push(ans * 8 + i);
        }
    }

    anss
}

fn hacky_magic(expected: u64, a: u64) -> Vec<u64> {
    let mut opts = Vec::new();

    for b in 0..8 {
        if (((b ^ 5) ^ ((a + b) >> (b ^ 5))) ^ 6) % 8 == expected {
            opts.push(b);
        }
    }

    opts
}

fn solve(prg: &[u64], pos: usize, cur: u64, i: Vec<u64>, out: &mut Vec<u64>) {
    let sol = solution(&[cur, 0, 0], &prg);

    if pos > 9 {
        let sols = find(prg.len(), &i[..9]);
        let ans = brute(prg, &sols);

        if ans != u64::MAX {
            out.push(ans);
        }
    }

    if pos >= prg.len() {
        return;
    }

    for opt in hacky_magic(prg[prg.len() - 1 - pos], cur) {
        let mut cp = i.clone();
        cp.push(opt);

        solve(prg, pos + 1, (cur + opt) * 8, cp, out);
    }
}

fn main() {
    let mut a = 0u64;
    let prg = [2u64,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0];

    let mut out = Vec::new();
    solve(&prg, 0, 0, vec![], &mut out);

    println!("{:?}", out);
    println!("{}", *out.iter().min().unwrap());

    //let out = brute(&prg, &f(prg.len(), &prg));
    //println!("{out}");
    // b = a % 8
    //
    // b1 = b ^ 5
    // c = (a << (b ^ 5))
    // b2 = ((b ^ 5) ^ (a << (b ^ 5)))
    // b3 = (((b ^ 5) ^ (a << (b ^ 5))) ^ 6)

    // [7, 7, 5, 6, 5, 7, 5, 4, 3, 1, 3, 2, 3, 5, 5, 3, 0]
    //                          105734774296321
    //                          105734774296321
    println!("{:?}", solution(&[105734774296321, 0, 0], &prg));

    //Register A: 61156655
    //Register B: 0
    //Register C: 0

    //Program: 2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0

    // 2.4 => bst | b
    // 1.5 => bxl | b
    // 7.5 => cdv | c
    // 4.3 => bxc | b
    // 1.6 => bxl | b
    //
    // 0.3 => adv | a / 8
    //let prg = Vec::from([2,4,1,5,7,5,4,3,1,6,0,3,5,5,3,0]);


    //let mut f = vec![1; prg.len()];

    //loop {
    //    let mut a = a;
    //    let mut i = 0;
    //    let mut b;
    //    let mut c;

    //    while a != 0 {
    //        b = a % 8;
    //        b ^= 5;
    //        c = a / 2u64.pow(b as u64);
    //        b ^= c;
    //        b ^= 6;

    //        if b % 8 != prg[i] {
    //            continue 'out;
    //        }

    //        i += 1;
    //        a /= 8;
    //    }

    //    if i == prg.len() {
    //        println!("{a}");
    //    }
    //}
}
