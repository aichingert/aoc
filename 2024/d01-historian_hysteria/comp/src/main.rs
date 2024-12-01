fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/01").unwrap();

    let mut l = Vec::new();
    let mut r = Vec::new();

    for line in inp.lines() {
        println!("{line}");
        let s = line.split(" ").filter(|n| *n != "").collect::<Vec<_>>();

        if s.len() == 0 {
            continue;
        }

        println!("{s:?}");

        l.push(s[0].parse::<i32>().unwrap());
        r.push(s[1].parse::<i32>().unwrap());
    }

    let mut p1 = 0;
    let mut p2 = 0;
    let l2 = l.clone();
    let r2 = r.clone();

    l.sort();
    r.sort();

    for i in 0..l.len() {
        p1 += (l[i] - r[i]).abs();
    }

    for i in 0..l2.len() {
        let mut cnt = 0;

        for j in 0..r2.len() {
            if l2[i] == r2[j] {
                cnt += 1;
            }
        }

        p2 += l2[i] * cnt;
    }

    println!("{p1}");
    println!("{p2}");
}
