fn main() {
    let inp = std::fs::read_to_string("../input/06").unwrap().trim().to_string();

    let num = inp.lines().map(|l| l.split_whitespace().skip(1).collect::<Vec<_>>()).collect::<Vec<_>>();

    let inp = inp.lines().map(|l| l.split_whitespace().skip(1).map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut t = String::new();
    let mut d = String::new();

    for i in 0..num[0].len() {
        t.push_str(num[0][i]);
        d.push_str(num[1][i]);
    }

    let (t, d) = (t.parse::<u64>().unwrap(), d.parse::<u64>().unwrap());

    let (mut s, mut e) = (0, 0);
    for w in 1..=t {
        if (t - w) * w > d {
            if s == 0 {
                s = w;
            }
            e = w;
        }
    }

    println!("{:?}", (e + 1 - s));

    let mut ans = 1;
    for i in 0..inp[0].len() {
        let (mut s, mut e) = (0, 0);

        for w in 1..=inp[0][i] {
            if (inp[0][i] - w) * w > inp[1][i] {
                if s == 0 {
                    s = w;
                }
                e = w;
            }
        }

        ans = ans * ((e + 1) - s);
    }

    println!("{ans}");
}
