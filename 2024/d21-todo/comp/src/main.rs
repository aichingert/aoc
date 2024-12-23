use std::collections::HashMap;

// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

const NUM: [[char; 5]; 6] = [
    ['#','#','#','#', '#'],
    ['#', '7', '8', '9', '#'], 
    ['#', '4', '5', '6', '#'], 
    ['#', '1', '2', '3', '#'], 
    ['#', '#', '0', 'A', '#'],
    ['#','#','#','#', '#']
];

fn sol(
    map: &mut HashMap<(usize, usize, usize), usize>, 
    code: &[char], 
    (py, px): (usize, usize), 
    mut steps: Vec<char>,
) -> Vec<Vec<char>> {
    if code.is_empty() {
        return vec![steps];
    }

    if code[0] == NUM[py][px] {
        steps.push('A');
        map.insert((code.len() - 1, py, px), steps.len());
        return sol(map, &code[1..], (py, px), steps);
    }
    map.insert((code.len(), py, px), steps.len());
    let mut opts = Vec::new();

    for (v, dy, dx) in [('v', 1, 0), ('^', -1, 0), ('<', 0, -1), ('>', 0, 1)] {
        let (ny, nx) = ((py as i32 + dy) as usize, (px as i32 + dx) as usize);

        if NUM[ny][nx] == '#' {
            continue;
        }
        if let Some(&len) = map.get(&(code.len(), ny, nx)) {
            if len < steps.len() + 1 {
                continue;
            }
        }

        let mut cp = steps.clone();
        cp.push(v);

        for ans in sol(map, code, (ny, nx), cp) {
            opts.push(ans);
        }
    }

    opts
}

fn find<'a>(
    cache: &mut HashMap<(&'a [char], u8), usize>,
    lookup: &'a HashMap<(char, char), Vec<Vec<char>>>,
    seq: &'a [char],
    depth: u8,
) -> usize {
    if let Some(&ans) = cache.get(&(seq, depth)) {
        return ans;
    }

    let mut s = 'A';
    let mut ans = 0;

    if depth == 1 {
        for &e in seq {
            ans += lookup.get(&(s, e)).unwrap()[0].len();
            s = e;
        }

        cache.insert((seq, depth), ans);
        return ans;
    }

    for &e in seq {
        let mut min_dis = usize::MAX;

        for subseq in lookup.get(&(s, e)).unwrap() {
            min_dis = min_dis.min(find(cache, lookup, subseq, depth -1));
        }

        s = e;
        ans += min_dis;
    }

    cache.insert((seq, depth), ans);
    ans
}

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/21").unwrap();
    let lookup = HashMap::from([
        (('A', '>'), vec![vec!['v', 'A']]),
        (('A', '^'), vec![vec!['<', 'A']]),
        (('A', 'v'), vec![vec!['v', '<', 'A'], vec!['<', 'v', 'A']]),
        (('A', '<'), vec![vec!['v', '<', '<', 'A'], vec!['<', 'v', '<', 'A']]),
        (('A', 'A'), vec![vec!['A']]),
        (('^', 'A'), vec![vec!['>', 'A']]),
        (('^', 'v'), vec![vec!['v', 'A']]),
        (('^', '<'), vec![vec!['v', '<', 'A']]),
        (('^', '>'), vec![vec!['v', '>', 'A'], vec!['>', 'v', 'A']]),
        (('^', '^'), vec![vec!['A']]),
        (('v', 'A'), vec![vec!['>', '^', 'A'], vec!['^', '>', 'A']]),
        (('v', '^'), vec![vec!['^', 'A']]),
        (('v', '<'), vec![vec!['<', 'A']]),
        (('v', '>'), vec![vec!['>', 'A']]),
        (('v', 'v'), vec![vec!['A']]),
        (('>', 'A'), vec![vec!['^', 'A']]),
        (('>', '^'), vec![vec!['^', '<', 'A'], vec!['<', '^', 'A']]),
        (('>', '<'), vec![vec!['<', '<', 'A']]),
        (('>', 'v'), vec![vec!['<', 'A']]),
        (('>', '>'), vec![vec!['A']]),
        (('<', 'A'), vec![vec!['>', '>', '^', 'A'], vec!['>', '^', '>', 'A']]),
        (('<', '^'), vec![vec!['>', '^', 'A']]),
        (('<', '>'), vec![vec!['>', '>', 'A']]),
        (('<', 'v'), vec![vec!['>', 'A']]),
        (('<', '<'), vec![vec!['A']]),
    ]);

    let (mut p1, mut p2) = (0, 0);

    for code in inp.lines().filter(|n| !n.is_empty()) {
        let ch = code.chars().collect::<Vec<_>>(); 
        let n = ch.iter().filter(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap();

        let paths = sol(&mut HashMap::new(), &ch, (4, 3), Vec::new());
        let min_p_len = paths.iter().map(|p| p.len()).min().unwrap();
        let paths = paths.into_iter().filter(|p| p.len() == min_p_len).collect::<Vec<_>>();

        let mut part_one = usize::MAX;
        let mut part_two = usize::MAX;

        for path in paths {
            part_one = part_one.min(find(&mut HashMap::new(), &lookup, &path, 2));
            part_two = part_two.min(find(&mut HashMap::new(), &lookup, &path, 25));
        }

        p1 += part_one * n;
        p2 += part_two * n;
    }

    println!("p1: {p1}");
    println!("p2: {p2}");
}
