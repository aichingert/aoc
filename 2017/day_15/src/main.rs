fn main() {
    solve_part_one();
    solve_part_two();
}

fn solve_part_one() {
    let mut gen_a: i64 = 65;
    let mut gen_b: i64 = 8921;

    let fa: i64 = 16807;
    let fb: i64 = 48271;
    let divide: i64 = 2147483647;
    let mut count: i32 = 0;

    for _ in 1..40000000 {
        gen_a = (gen_a * fa) % divide;
        gen_b = (gen_b * fb) % divide;
        
        let a = to_bits(gen_a);
        let b = to_bits(gen_b);

        if a == b {
            count += 1;
        }
    }

    println!("Solve part 1: {}", count);
}

fn solve_part_two() {
    let mut gen_a: i64 = 634;
    let mut gen_b: i64 = 301;

    let fa: i64 = 16807;
    let fb: i64 = 48271;
    let divide: i64 = 2147483647;
    let mut count: i32 = 0;

    for _ in 1..5000000 {
        gen_a = (gen_a * fa) % divide;
        gen_b = (gen_b * fb) % divide;

        while gen_a % 4 != 0 {
            gen_a = (gen_a * fa) % divide;
        }

        while gen_b % 8 != 0 {
            gen_b = (gen_b * fb) % divide;
        }
        
        let a = to_bits(gen_a);
        let b = to_bits(gen_b);

        if a == b {
            count += 1;
        }
    }

    println!("Solve part 2: {}", count);
}

fn to_bits(number: i64) -> String {
    let mut bits: String = String::new();
    let mut cur_num: i64 = number;

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