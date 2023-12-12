use std::collections::HashMap;

fn n_ways(memo: &mut HashMap<(usize, usize), u128>, s: &[char], n: &[usize], mut sp: usize, np: usize) -> u128 {
    if let Some(res) = memo.get(&(sp, np)) {
        return *res;
    }

    while sp < s.len() && s[sp] == '.' { sp += 1; }

    if sp < s.len() && s[sp] == '#' {
        if fits(s, n, sp, np) {
            let (sx, nx) = (sp + n[np] + 1, np + 1);
            let res = n_ways(memo, s, n, sx, nx);
            memo.insert((sx, nx), res);
            res
        } else {
            0
        }
    } else {
        if sp >= s.len() { return if np >= n.len() { 1 } else { 0 } }

        let skip_res = n_ways(memo, s, n, sp + 1, np);
        memo.insert((sp + 1, np), skip_res);

        if fits(s, n, sp, np) {
            let take_res = n_ways(memo, s, n, sp + n[np] + 1, np + 1);
            memo.insert((sp + n[np] + 1, np + 1), take_res);

            skip_res + take_res
        } else {
            skip_res
        }
    }
}

fn fits(s: &[char], n: &[usize], mut sp: usize, np: usize) -> bool {
    if np >= n.len() { return false; }
    if sp + n[np] > s.len() { return false; }

    let mut i = n[np];
    while i > 0 && s[sp] != '.' { i -= 1; sp += 1; }

    i == 0 && !(sp < s.len() && s[sp] == '#')
}

fn main() {
    let inp = std::fs::read_to_string("../input/12").unwrap().trim().to_string();
    let (mut p1, mut p2) = (0, 0);

    for line in inp.lines() {
        println!("{line}");
        let (chs, nms) = line.split_once(" ").unwrap();
        let (chs, nms) = (
            chs.chars().collect::<Vec<char>>(),
            nms.split(',').map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>()
        );

        let mut cha = chs.clone();
        let mut nma = nms.clone();

        for i in 0..4 {
            cha.push('?');
            cha.extend_from_slice(&chs);
            nma.extend_from_slice(&nms);
        }

        p1 += n_ways(&mut HashMap::new(), &chs, &nms, 0, 0);
        p2 += n_ways(&mut HashMap::new(), &cha, &nma, 0, 0);
    }

    println!("Part one: {p1}");
    println!("Part two: {p2}");
}
