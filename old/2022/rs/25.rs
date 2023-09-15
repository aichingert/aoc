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
    let mut factor = 1;
    let mut cur = 0;

    while factor * 2 + cur < dec {
        cur += factor * 2;
        factor *= 5;
    }

    let (mut result, mut remaining) = if factor + cur >= dec {
        ("1".to_string(), factor)
    } else {
        ("2".to_string(), 2 * factor)
    };
    factor /= 5;
    cur -= 2 * factor;

    while factor > 0 {
        result.push(if remaining == dec {
            '0'
        } else if remaining < dec {
            let rpf = remaining + factor;

            if rpf >= dec || rpf + cur >= dec {
                remaining = rpf;
                '1'
            } else {
                remaining = rpf + factor;
                '2'
            }
        } else {
            let rmf = remaining - factor;

            if rmf + cur < dec {
                '0'
            } else if rmf <= dec || rmf - cur <= dec {
                remaining = rmf;
                '-'
            } else {
                remaining = rmf - factor;
                '='
            }
        });

        factor /= 5;
        cur -= 2 * factor;
    }

    result
}

#[cfg(test)]
mod tests {
    use dec_to_snafu;

    #[test]
    fn test_to_ten() {
        assert_eq!(dec_to_snafu(1), "1".to_string());
        assert_eq!(dec_to_snafu(2), "2".to_string());
        assert_eq!(dec_to_snafu(3), "1=".to_string());
        assert_eq!(dec_to_snafu(4), "1-".to_string());
        assert_eq!(dec_to_snafu(5), "10".to_string());
        assert_eq!(dec_to_snafu(6), "11".to_string());
        assert_eq!(dec_to_snafu(7), "12".to_string());
        assert_eq!(dec_to_snafu(8), "2=".to_string());
        assert_eq!(dec_to_snafu(9), "2-".to_string());
        assert_eq!(dec_to_snafu(10), "20".to_string()); 
    }

    #[test]
    fn test_to_fourth_factor() {
        assert_eq!(dec_to_snafu(11), "21".to_string());
        assert_eq!(dec_to_snafu(12), "22".to_string());
        assert_eq!(dec_to_snafu(13), "1==".to_string());
        assert_eq!(dec_to_snafu(14), "1=-".to_string());
        assert_eq!(dec_to_snafu(15), "1=0".to_string());
        assert_eq!(dec_to_snafu(16), "1=1".to_string());
        assert_eq!(dec_to_snafu(17), "1=2".to_string());
        assert_eq!(dec_to_snafu(18), "1-=".to_string());
        assert_eq!(dec_to_snafu(19), "1--".to_string());
        assert_eq!(dec_to_snafu(20), "1-0".to_string());
        assert_eq!(dec_to_snafu(62), "222".to_string());
        assert_eq!(dec_to_snafu(63), "1===".to_string());
    }

    #[test]
    fn test_high_numbers() {
        assert_eq!(dec_to_snafu(2022), "1=11-2".to_string());
        assert_eq!(dec_to_snafu(12345), "1-0---0".to_string());
        assert_eq!(dec_to_snafu(314159265), "1121-1110-1=0".to_string());
    }
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

    println!("Part one: {}", sum);
    println!("Part two: {}", dec_to_snafu(sum));
}
