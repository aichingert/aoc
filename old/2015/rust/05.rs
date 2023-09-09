// Advent of Code 2015, day 5
// (c) aichingert

const I: [&str; 4] = ["ab", "cd", "pq", "xy"];

fn part1(inp: &str) -> u32 {
    let mut c: u32 = 0;

    for s in inp.lines() {
        let chs = s.chars().collect::<Vec<char>>();
        let mut vc: u32 = if is_vowel(chs[chs.len() - 1]) { 1 } else { 0 };
        let mut v: bool = true;
        let mut dd: bool = false;
        let mut s: String = String::from(chs[0]);

        for i in 0..chs.len() - 1 {
            if is_vowel(chs[i]) {
                vc += 1;
            }
            s.push(chs[i + 1]);

            if I.contains(&s.as_str()) {
                v = false;
                break;
            }

            if chs[i] == chs[i + 1] {
                dd = true;
            }
            s.remove(0);
        }

        if v && dd && vc > 2 {
            c += 1;
        }
    }

    c
}

fn part2(inp: &str) -> u32 {
    let mut c: u32 = 0;

    for s in inp.lines() {
        let chs = s.chars().collect::<Vec<char>>();
        let mut f: bool = false;
        let mut s: bool = false;

        for i in 0..chs.len() - 2 {
            for j in i + 2..chs.len() - 1 {
                if chs[i] == chs[j] && chs[i + 1] == chs[j + 1] {
                    f = true;
                }
            }

            if chs[i] == chs[i + 2] {
                s = true;
            }
        }

        if f && s {
            c += 1;
        }
    }

    c
}

fn is_vowel(ch: char) -> bool {
    ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u'
}

fn main() {
    let input = std::fs::read_to_string("../input/05").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
