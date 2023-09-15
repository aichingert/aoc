fn snafu_to_dec(snafu: &str) -> i64 {
    let mut sum = 0;
    let digits = snafu.chars().collect::<Vec<_>>();
    let len: usize = digits.len();
    let mut fac = 1;

    for i in 1..=len {
        sum += get_value(digits[len - i], fac);
        fac *= 5;
    }

    sum
}

fn dec_to_snafu(dec: i64) -> String {
    "".to_string()
}

fn get_value(ch: char, fac: i64) -> i64 {
    fac * match ch {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '-' => -1,
        '=' => -2,
        _ => panic!("not a snafu number"),
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/25").unwrap().trim().to_string();
    let inp = inp
        .lines()
        .collect::<Vec<_>>();

    let mut sum = 0;

    for i in 0..inp.len() {
        sum += snafu_to_dec(inp[i]);
    }

    println!("{:?}", sum);
    println!("{:?}", dec_to_snafu(sum));
}
