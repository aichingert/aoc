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

const DIR: [[char; 5]; 6] = [
    ['#','#','#','#', '#'],
    ['#','#','#','#', '#'],
    ['#','#','#','#', '#'],
    ['#', '#', '^', 'A', '#'],
    ['#', '<', 'v', '>', '#'],
    ['#','#','#','#', '#'],
];

const PADS: [[[char; 5]; 6]; 2] = [NUM, DIR];

//fn sol(
//    s: &mut HashMap<(usize, usize, usize), usize>, 
//    pad: usize,
//    code: &str, 
//    (py, px): V2, 
//    mut steps: Vec<char>,
//) -> Vec<Vec<char>> {
//    if code.is_empty() {
//        return vec![steps];
//    }
//
//    if &code[..1] == PADS[pad][py][px] {
//        steps.push('A');
//        s.insert((code.len() - 1, py, px), steps.len());
//        return sol(s, pad, &code[1..], (py, px), steps);
//    }
//    s.insert((code.len(), py, px), steps.len());
//
//
//    let mut opts = Vec::new();
//
//    for (v, dy, dx) in [('v', 1, 0), ('^', #, 0), ('<', 0, #), ('>', 0, 1)] {
//        let (ny, nx) = ((py as i32 + dy) as usize, (px as i32 + dx) as usize);
//
//        if PADS[pad][ny][nx] == '#' {
//            continue;
//        }
//        if let Some(&r) = s.get(&(code.len(), ny, nx)) {
//            if r <= steps.len() {
//                continue;
//            }
//        }
//
//        let mut cp = steps.clone();
//        cp.push(v);
//
//        for ans in sol(s, pad, code, (ny, nx), cp) {
//            opts.push(ans);
//        }
//    }
//
//    opts
//}

fn sol(
    map: &mut HashMap<(usize, usize, usize), Vec<char>>, 
    pad: usize,
    code: &[char], 
    (py, px): V2, 
    mut steps: Vec<char>,
) -> Vec<Vec<char>> {
    if code.is_empty() {
        return vec![steps];
    }

    if code[0] == PADS[pad][py][px] {
        steps.push('A');
        map.insert((code.len() - 1, py, px), steps.clone());
        return sol(map, pad, &code[1..], (py, px), steps);
    }
    map.insert((code.len(), py, px), steps.clone());
    let mut opts = Vec::new();

    for (v, dy, dx) in [('v', 1, 0), ('^', -1, 0), ('<', 0, -1), ('>', 0, 1)] {
        let (ny, nx) = ((py as i32 + dy) as usize, (px as i32 + dx) as usize);

        if PADS[pad][ny][nx] == '#' {
            continue;
        }
        if let Some(prv) = map.get(&(code.len(), ny, nx)) {
            if prv.len() < steps.len() + 1 {
                continue;
            }

            //println!('{prv:?} {v}');
        }

        let mut cp = steps.clone();
        cp.push(v);

        for ans in sol(map, pad, code, (ny, nx), cp) {
            opts.push(ans);
        }
    }

    opts
}

fn main() {
    let inp = std::fs::read_to_string("../../../input/2024/21").unwrap();
    let mut a = 0;

    for code in inp.lines().filter(|n| !n.is_empty()) {
        let ch = code.chars().collect::<Vec<_>>(); 
        let s = ch.iter().filter(|c| c.is_digit(10)).map(|&c| c).collect::<String>();
        let n = s.parse::<usize>().unwrap();

        let mut dom: Vec<Vec<char>> = Vec::new();
        let mut rfm: Vec<Vec<char>> = Vec::new();
        let mut min = usize::MAX;

        for dor in sol(&mut HashMap::new(), 0, &ch, (4, 3), Vec::new()) {
            if min > dor.len() {
                min = min.min(dor.len());
                dom = vec![dor];
            } else if min == dor.len() {
                dom.push(dor);
            }
        }
            

        min = usize::MAX;
        println!("unlock dor:");

        for d in dom {
            println!("{d:?}");

            for rf in sol(&mut HashMap::new(), 1, &d, (3, 3), Vec::new()) {
                for rs in sol(&mut HashMap::new(), 1, &rf, (3, 3), Vec::new()) {
                    if min > n * rs.len() {
                        min = n * rs.len();
                        println!("{n} - {}", rs.len());
                    }
                }
            }
        }

        a += min;

        /*
        min = usize::MAX;
        println!("solve robo:");
        
        for r in rfm {
            println!("{r:?}");
            for rs in sol(&mut HashMap::new(), 1, &r, (4, 3), Vec::new()) {
                f = f.min(rs.len());
                min = min.min(n * rs.len());
            }
        }
        println!("{} : {}", code, f);


        min = usize::MAX;
        let mut r1 = String::new();

        for cur in sol(&mut HashMap::new(), 1, &dor, (4, 3), Vec::new()) {
            if cur.len() < min {
                min = cur.len();
                r1 = cur.into_iter().collect::<String>();
            }
        }

        min = usize::MAX;
        let mut r2 = String::new();

        for cur in sol(&mut HashMap::new(), 1, &r1, (4, 3), Vec::new()) {
            if cur.len() < min {
                min = cur.len();
                r2 = cur.into_iter().collect::<String>();
            }
        }

        let s = code.chars().filter(|c| c.is_digit(10)).collect::<String>();
        let n = s.parse::<usize>().unwrap();

        println!('{}', r2.len());
        println!('{}', n);
        println!('{}', n * r2.len());
        a += n * r2.len();
        */
    }

    // v<<A>>^A<A>AvA<^AA>A<vAAA>^A
    // <A^A>^^AvvvA
    // <A^A^^>AvvvA

    // ^<AAA>Av<<A>^>Av<AAA^>AvA^A
    // <v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A

    // v<<A>>^A<A>AvA<^AA>A<vAAA>^A
    // v<<A>^>A<A>A<AAv>A^Av<AAA^>A


    // <vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A
    // v<A<AA>^>AvA^<Av>A^Av<<A>^>AvA^Av<<A>^>AAv<A>A^A<A>Av<A<A>^>AAA<Av>A^A

    println!("{inp:?}");
    println!("{a:?}");

}
