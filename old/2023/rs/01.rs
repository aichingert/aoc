fn part_one(inp: &Vec<Vec<char>>) -> u32 {
    let mut ans = 0;
    let (mut first, mut last) = (10, 0);

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            if inp[i][j] >= '0' && inp[i][j] <= '9' {
                if first > 9 {
                    first = (inp[i][j] as u8 - b'0') as u32;
                }
                last = (inp[i][j] as u8 - b'0') as u32;
            }
        }

        ans += first * 10 + last;
        first = 10;
    }

    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/01").unwrap().trim().to_string();
    let inp = inp.lines().map(|n| n.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("Part one: {}", part_one(&inp));
}
