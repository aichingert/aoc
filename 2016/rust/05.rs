// Advent of Code 2016, day 5
// (c) aichingert

#[path="../../utils/rust/md5.rs"] mod md5;

fn part1(inp: &str) -> String {
    let mut ans = String::new();
    let mut n = 0;

    'outer: loop {
        n += 1;
        let s = md5::md5_utf8((inp.to_owned() + &n.to_string()).as_str()).chars().collect::<Vec<char>>();
        
        for ch in s[..5].iter() {
            if ch != &'0' { continue 'outer; }
        }

        ans.push(s[5]);

        if ans.len() > 7 {
            return ans;
        }
    }
}

fn part2(inp: &str) -> String {
    let mut ans: Vec<char> = vec!['_';8];
    let mut n = 0;

    'outer: loop {
        n += 1;
        let s = md5::md5_utf8((inp.to_owned() + &n.to_string()).as_str()).chars().collect::<Vec<char>>();
        
        for ch in s[..5].iter() {
            if ch != &'0' { continue 'outer; }
        }

        let n: usize = ((s[5] as u8) - 48) as usize;

        if n < 8 && ans[n] == '_' {
            ans[n] = s[6];
        }

        if !ans.contains(&'_') {
            return ans.into_iter().collect::<String>();
        }
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/05").unwrap();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
