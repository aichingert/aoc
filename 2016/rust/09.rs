// Advent of Code 2016, day 9
// (c) aichingert

fn solve(inp: &[char], part1: bool) -> usize {
    let mut i = 0;
    let mut l = 0;
    
    while i < inp.len() {
        if inp[i] == '(' {
            let mut f = Vec::<char>::new();
            let mut s = Vec::<char>::new();

            while i < inp.len() {
                i += 1;

                if inp[i] == 'x' { break; }
                f.push(inp[i]);
            }

            while i < inp.len() {
                i += 1;

                if inp[i] == ')' { break; }
                s.push(inp[i]);
            }

            let f = f.iter().collect::<String>().parse::<usize>().unwrap();
            let s = s.iter().collect::<String>().parse::<usize>().unwrap();

            if part1 {
                i += f;
                l += s * f - 1;
            } else {
                l += s * solve(&inp[i+1..i+f+1], part1) - 1;
                i += f;
            }
        }
        i += 1;
        l += 1;
    }

    l
}

fn main() {
    let inp = std::fs::read_to_string("../input/09").unwrap().chars().collect::<Vec<char>>();

    println!("Part 1: {}", solve(&inp, true));
    println!("Part 2: {}", solve(&inp, false));
}