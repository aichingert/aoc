fn main() {
    let dsk = std::fs::read_to_string("../../../input/2024/09")
        .unwrap()
        .bytes()
        .filter(|&b| b != 10)
        .map(|b| (b - b'0') as i8)
        .collect::<Vec<_>>();

    let mut lst = Vec::new();
    let mut a = 0;

    for (i, &f) in dsk.iter().enumerate() {
        for _ in 0..f {
            if i & 1 == 0 {
                lst.push((i / 2) as i64);
            } else {
                lst.push(-1);
            }
        }
    }

    let (mut lhs, mut rhs) = (0, lst.len() - 1);
    let cp = lst.clone();

    while lhs < rhs {
        while lst[lhs] != -1 {
            lhs += 1;
        }
        while lst[rhs] == -1 {
            rhs -= 1;
        }

        if lhs >= rhs {
            break;
        }

        lst[lhs] = lst[rhs];
        lst[rhs] = -1;

        lhs += 1;
    }

    for i in 0..lst.len() {
        if lst[i] == -1 {
            break;
        }

        a += (i as i64) * lst[i];
    }

    println!("p1: {a:?}");

    let id = (dsk.len() / 2) as i64;
    lst = cp;

    let mut b = lst.len() - 1;

    for i in (0..=id).rev() {
        let mut rhs = 0;

        while rhs < lst.len() && lst[rhs] != i {
            rhs += 1;
        }

        let start = rhs;

        while rhs < lst.len() && lst[rhs] == i {
            rhs += 1;
        }

        let mut lhs = 0;

        while lhs < b {
            while lhs < b && lst[lhs] != -1 {
                lhs += 1;
            }

            let s = lhs;
            while lhs < b && lst[lhs] == -1 {
                lhs += 1;
            }

            if lhs - s >= rhs - start {
                for j in 0..(rhs - start) {
                    lst[s + j] = i;
                    lst[start + j] = -1;
                }

                break;
            }
        }

        b = start;
    }

    a = 0;

    for (scale, &num) in lst.iter().enumerate() {
        if num != -1 {
            a += (scale as i64) * num;
        }
    }

    println!("p2: {a}");
}
