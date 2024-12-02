fn inc(a: &[i32]) -> Option<usize> {
    for i in 0..a.len() - 1 {
        if  !(a[i] > a[i + 1] && a[i] - a[i + 1] <= 3) {
            return Some(i + 1);
        }
    }

    None
}

fn rem(a: &[i32], i: usize) -> bool {
    let mut r = Vec::new();
    r.extend_from_slice(&a[..i]);
    r.extend_from_slice(&a[i + 1..]);

    println!("{a:?} - {r:?}");

    inc(&r).is_none()
}

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/02").unwrap();

    let mut p1 = 0;
    let mut p2 = 0;

    for line in inp.lines().filter(|s| !s.is_empty()) {
        let l = line.split(' ').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();

        if inc(&l).is_none() || inc(&l.iter().copied().rev().collect::<Vec<_>>()).is_none() {
            p1 += 1;
            p2 += 1;
            continue;
        }

        let r = l.iter().copied().rev().collect::<Vec<_>>();
        let fi = inc(&l).unwrap();
        let si = inc(&r).unwrap();

        if rem(&l, fi) || rem(&l, fi - 1) || rem(&r, si) || rem(&r, si - 1) {
            p2 += 1;
        }
    }

    println!("{p1}");
    println!("{p2}");
}
