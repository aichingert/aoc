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

type V2 = (usize, usize);

const NUM: [[char; 5]; 6] = [
    ['#','#','#','#', '#'],
    ['#', '7', '8', '9', '#'], 
    ['#', '4', '5', '6', '#'], 
    ['#', '1', '2', '3', '#'], 
    ['#', '#', '0', 'A', '#'],
    ['#','#','#','#', '#']
];

const DIR: [[char; 5]; 4] = [
    ['#','#','#','#', '#'],
    ['#', '#', '^', 'A', '#'],
    ['#', '<', 'v', '>', '#'],
    ['#','#','#','#', '#'],
];

fn sol(
    map: &mut HashMap<(usize, usize, usize), usize>, 
    code: &[char], 
    (py, px): V2, 
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

fn chain(lookup: &HashMap<(char, char), Vec<char>>, robo: u32, src: char, dst: &[char]) -> usize {
    if robo == 0 {
        return lookup.get(&(src, dst[dst.len() - 1])).unwrap().len();
    }

    let (mut size, mut nxt) = (0, 'A');

    for &cur in dst {
        let path = lookup.get(&(nxt, cur)).unwrap();

        size += chain(lookup, robo - 1, cur, path);
        nxt = cur;
    }

    size
}

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/21").unwrap();
    let mut a = 0;

    for code in inp.lines().filter(|n| !n.is_empty()) {
        let ch = code.chars().collect::<Vec<_>>(); 
        let s = ch.iter().filter(|c| c.is_digit(10)).map(|&c| c).collect::<String>();
        let n = s.parse::<usize>().unwrap();

        let mut dom = Vec::new();
        let mut min = usize::MAX;

        // ['#','#','#','#', '#'],
        // ['#', '#', '^', 'A', '#'],
        // ['#', '<', 'v', '>', '#'],
        // ['#','#','#','#', '#'],

        let lookup = HashMap::from([
            (('A', '>'), vec!['v', 'A']),
            (('A', '^'), vec!['<', 'A']),
            (('A', 'v'), vec!['v', '<', 'A']),
            (('A', '<'), vec!['v', '<', '<', 'A']),
            (('A', 'A'), vec!['A']),

            (('^', 'A'), vec!['>', 'A']),
            (('^', 'v'), vec!['v', 'A']),
            (('^', '<'), vec!['v', '<', 'A']),
            (('^', '>'), vec!['v', '>', 'A']),
            (('^', '^'), vec!['A']),

            (('v', 'A'), vec!['>', '^', 'A']),
            (('v', '^'), vec!['^', 'A']),
            (('v', '<'), vec!['<', 'A']),
            (('v', '>'), vec!['>', 'A']),
            (('v', 'v'), vec!['A']),

            (('>', 'A'), vec!['^', 'A']),
            (('>', '^'), vec!['^', '<', 'A']),
            (('>', '<'), vec!['<', '<', 'A']),
            (('>', 'v'), vec!['<', 'A']),
            (('>', '>'), vec!['A']),

            (('<', 'A'), vec!['>', '>', '^', 'A']),
            (('<', '^'), vec!['>', '^', 'A']),
            (('<', '>'), vec!['>', '>', 'A']),
            (('<', 'v'), vec!['>', 'A']),
            (('<', '<'), vec!['A']),
        ]);

        for dor in sol(&mut HashMap::new(), &ch, (4, 3), Vec::new()) {
            if min > dor.len() {
                min = dor.len();
                dom = vec![dor];
            } else if min == dor.len() {
                dom.push(dor);
            }
        }

        min = usize::MAX;

        for mut door in dom {
            //let size = chain(&lookup, 25, 'A', &door);

            //println!("{code} - {:?}", size);
            //min = min.min(size);

            for i in 0..2 {
                let mut next = Vec::new();
                let mut start = 'A';

                for &c in &door {
                    let path = lookup.get(&(start, c)).unwrap();
                    start = c;

                    next.extend_from_slice(path);
                }

                door = next;
                println!("{}", door.len());
            }

            println!("{code} - {:?}", door.len());
            min = min.min(door.len());
        }

        a += min * n;
    }

    println!("{inp:?}");
    println!("{a:?}");

}
