// Advent of Code 2015, day 12
// (c) aichingert

const V: [char; 11] = ['-', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn part1(ch: &[char]) -> i32 {
    let mut cur = String::new();

    for i in 0..ch.len() {
        if V.contains(&ch[i]) {
            cur.push(ch[i]);
        } else {
            cur.push(' ');
        }
    }

    cur.split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn part2(ch: &[char], cur: &mut usize) -> i32 {
    *cur += 1;
    let mut ans = 0;
    let mut s = String::new();

    while *cur < ch.len() - 2 {
        if ch[*cur] == 'r' && ch[*cur + 1] == 'e' && ch[*cur + 2] == 'd' {
            let mut cc = 0;
            while cc != -1 {
                if ch[*cur] == '{' {
                    cc += 1;
                } else if ch[*cur] == '}' {
                    cc -= 1;
                }

                if cc > -1 {
                    *cur += 1;
                }
            }
            return 0;
        }

        ans += match ch[*cur] {
            '{' => part2(ch, cur),
            '}' => break,
            '[' => array(ch, cur),
            _ => 0,
        };

        if V.contains(&ch[*cur]) {
            s.push(ch[*cur]);
        } else {
            s.push(' ');
        }

        *cur += 1;
    }

    ans + s
        .split(' ')
        .filter(|st| !st.is_empty())
        .map(|st| st.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn array(ch: &[char], cur: &mut usize) -> i32 {
    *cur += 1;
    let mut ans = 0;
    let mut s = String::new();

    while *cur < ch.len() {
        ans += match ch[*cur] {
            '{' => part2(ch, cur),
            '[' => array(ch, cur),
            ']' => break,
            _ => 0,
        };

        if V.contains(&ch[*cur]) {
            s.push(ch[*cur]);
        } else {
            s.push(' ');
        }

        *cur += 1;
    }

    ans + s
        .split(' ')
        .filter(|st| !st.is_empty())
        .map(|st| st.parse::<i32>().unwrap())
        .sum::<i32>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/12").unwrap();
    let inp = inp.trim().chars().collect::<Vec<char>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp, &mut 0));
}
