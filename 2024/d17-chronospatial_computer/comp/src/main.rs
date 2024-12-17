fn combo(rgs: &[u64], op: u64) -> u64 {
    assert!(op < 7);

    if op <= 3 {
        op
    } else {
        rgs[op as usize - 4]
    }
}

// while A != 0 {
//     B := (A % 8) ^ 5
//     C := A / (2 ** B)
//     B := (B ^ C) ^ 6
//     A := A / 8
//     print(B)
// }

// Register A: 61156655
// Register B: 0
// Register C: 0
// Program: 2,4,1,5, . 7,5, . 4,3, . 1,6, . 0,3, 5,5,3,0

fn solf(prg: &[u64]) -> u64 {
    let mut a = 0u64;
    let mut b = 0u64;
    let mut c = 0u64;

    for i in (0..prg.len()).rev() {
        b += prg[i];
        a *= 8;


    }

    b
}

fn custom(prg: &[u64], inp: u64, l: &mut usize) -> bool {
    let mut a = inp;
    let mut b = 0;
    let mut c = 0;
    let mut i = 0;

    while a != 0 {
        b = (a % 8) ^ 5;

        // 0 , 1 , 2 , 3 , 4 , 5 , 6 , 7
        // 5 , 4 , 7 , 6 , 3 , 0 , 3 , 2

        // a / 5
        // 5 ^ (a / 5) ^ 6
        // 
        c = a / (2_u64.pow(b as u32));
        b = b ^ c ^ 6;
        a /= 8;

        if i >= prg.len() || b % 8 != prg[i] {
            if *l <= i {
                *l = i;
                println!("{i} b: {b} - inp: {inp}");
            }

            return false;
        }

        i += 1;
    }

    println!("{i} {inp} {b}");
    prg.len() == i
}

fn sol(rgs: &[u64], prg: &[u64]) -> Vec<u64> {
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

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/17").unwrap();
    let (regs, program) = inp.split_once("\n\n").unwrap();

    let mut rgs = regs.lines().map(|n| n.split(": ").skip(1).next().unwrap().parse::<u64>().unwrap()).collect::<Vec<_>>();
    let prg = program.split_once(": ").unwrap().1.split(",").map(|n| n.trim().parse::<u64>().unwrap()).collect::<Vec<_>>();

    println!("{rgs:?} {prg:?}");
    let mut l = 0usize;

    /*
    for i in 1u64..u64::MAX {
        if custom(&prg, i, &mut l) {
            println!("found: {i}");
            break;
        }
        /*
        let cp = [i, rgs[1], rgs[2]];

        let out = sol(&cp, &prg);
        if out == prg {
            println!("{i}");
            break;
        }
        */
    }
    */
    
    println!("u64 not enough :<");
    println!("{}", solf(&prg));
    println!("{}", sol(&rgs, &prg).iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","));
}
