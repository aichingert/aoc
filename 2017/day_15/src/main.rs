fn main() {
    solve_part_one();
    solve_part_two();
}

fn solve_part_one() {
    let mut gen_a: u64 = 65;
    let mut gen_b: u64 = 8921;

    let fa: u64 = 16807;
    let fb: u64 = 48271;
    let divide: u64 = 2147483647;
    let mut count: i32 = 0;

    for _ in 1..40000000 {
        gen_a = (gen_a * fa) % divide;
        gen_b = (gen_b * fb) % divide;
        
        let a = gen_a as u16;
        let b = gen_b as u16;

        if a == b {
            count += 1;
        }
    }

    println!("Solve part 1: {}", count);
}

fn solve_part_two() {
    let mut gen_a: u64 = 634;
    let mut gen_b: u64 = 301;

    let fa: u64 = 16807;
    let fb: u64 = 48271;
    let divide: u64 = 2147483647;
    let mut count: i32 = 0;

    for i in 1..5000000 {
        gen_a = (gen_a * fa) % divide;
        gen_b = (gen_b * fb) % divide;

        while gen_a % 4 != 0 {
            gen_a = (gen_a * fa) % divide;
        }

        while gen_b % 8 != 0 {
            gen_b = (gen_b * fb) % divide;
        }
        
        let a = gen_a as u16;
        let b = gen_b as u16;

        if a == b {
            count += 1;
        }
    }

    println!("Solve part 2: {}", count);
}

/* Not performant enough but right.
fn to_bits(number: u64) -> String {
    let mut bits: String = String::new();
    let mut cur_num: u64 = number;

    while cur_num > 0 {
        if cur_num == 1 {
            bits.push('1');
            break;
        }

        if cur_num % 2 == 0 {
            bits.push('0')
        } else {
            bits.push('1')
        }

        cur_num /= 2;
    }

    while bits.len() < 16 {
        bits.push('0');
    }

    while bits.len() > 16 {
        bits.pop();
    }

    bits
}
*/