/*
inp w
z = w + 12
inp w
z = z * 26 + w + 7
inp w
z = z * 26 + w + 8
inp w
z = z * 26 + w + 8
inp w
z = z * 26 + w + 16
inp w
x = (z % 26) - 16 != w
z = (z / 26) * (25 * x + 1) + (w + 12) * x
inp w
z = z * 26 + w + 8
inp w
x = (z % 26) - 11 != w
z = (z / 26) * (25 * x + 1) + (w + 13) * x
inp w
x = (z % 26) - 13 != w
z = (z / 26) * (25 * x + 1) + (w + 3) * x
inp w
z = (z * 26) + w + 13
inp w
x = (z % 26) - 8 != w
z = (z / 26) * (25 * x + 1) + ((w + 3) * x)
inp w
x = (z % 26) - 1 != w
z = (z / 26) * (25 * x + 1) + ((w + 9) * x)
inp w
x = (z % 26) - 4 != w
z = (z / 26) * (25 * x + 1) + ((w + 4) * x)
inp w
x = (z % 26) - 14 != w
z = (z / 26) * (25 * x + 1) + ((w + 13) * x)
*/

#[inline(always)]
fn get_w(cur: &mut i128) -> i128 {
    let w = *cur % 10;
    *cur /= 10;
    w
}

pub fn solve() {

    let mut start = 99_999_999_999_999i128;

    while start >= 11_111_111_111_111 {
        let mut cur = start;
        let mut z  = get_w(&mut cur) + 12;
        z = z * 26 + get_w(&mut cur) + 7;
        z = z * 26 + get_w(&mut cur) + 8;
        z = z * 26 + get_w(&mut cur) + 8;
        z = z * 26 + get_w(&mut cur) + 16;

        let w = get_w(&mut cur);
        let x = if (z % 26) - 16 != w { 1 } else { 0 };
        z = (z / 26) * (25 * x + 1) + (w + 12) * x;

        z = z * 26 + get_w(&mut cur) + 8;

        let w = get_w(&mut cur);
        let x = if (z % 26) - 11 != w { 1 } else { 0 };
        z = (z / 26) * (25 * x + 1) + (w + 13) * x;

        let w = get_w(&mut cur);
        let x = if (z % 26) - 13 != w { 1 } else { 0 };
        z = (z / 26) * (25 * x + 1) + (w + 3) * x;

        z = z * 26 + get_w(&mut cur) + 13;

        let w = get_w(&mut cur);
        let x = if (z % 26) - 8 != w { 1 } else { 0 };
        z = (z / 26) * (25 * x + 1) + (w + 3) * x;

        let w = get_w(&mut cur);
        let x = if (z % 26) - 1 != w { 1 } else { 0 };
        z = (z / 26) * (25 * x + 1) + (w + 9) * x;

        let w = get_w(&mut cur);
        let x = if (z % 26) - 4 != w { 1 } else { 0 };
        z = (z / 26) * (25 * x + 1) + (w + 4) * x;

        let w = get_w(&mut cur);
        let x = if (z % 26) - 14 != w { 1 } else { 0 };
        z = (z / 26) * (25 * x + 1) + (w + 13) * x;

        if z == 0 {
            break;
        }

        start -= 1;
    }

    println!("{start}");
}
