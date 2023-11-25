// Advent of Code 2018, day 20
// (c) aichingert

fn part_one(inp: &[char]) -> usize {
    let mut stack: Vec<char> = vec![];

    stack.len()
}

fn follow_option(inp: &[char], loc: usize) -> (Vec<char>, usize) {
    let mut stack: Vec<Vec<char>> = vec![vec![]];
    let mut i = loc + 1;

    while i < inp.len() - 1 {
        let cur = stack.last_mut().unwrap();

        match inp[i] {
            '(' => {
                let (longest_stack, next) = follow_option(inp, i);
                cur.extend_from_slice(&longest_stack);
                i += next - 1;
            }
            ')' => break,
            '|' => stack.push(vec![]),
            c => {
                if cur.len() > 0 && cur[cur.len() - 1] == rev(c) {
                    cur.pop();
                } else {
                    cur.push(c);
                }
            },
        };

        i += 1;
    }
    println!("{:?}", stack);

    let mut longest = (0usize, 0usize);
    for j in 0..stack.len() {
        if stack[j].len() > longest.0 {
            longest = (stack[j].len(), j);
        }
    }

    (stack[longest.1].clone(), i)
}

fn rev(ch: char) -> char {
    match ch {
        'N' => 'S',
        'S' => 'N',
        'E' => 'W',
        'W' => 'E',
        _ => panic!("invalid direction"),
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/20").unwrap().trim().to_string().chars().collect::<Vec<_>>();

    println!("{:?}", follow_option(&inp, 0).0);

    println!("Part one: {}", part_one(&inp));
}
