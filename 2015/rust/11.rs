// Advent of Code 2015, day 11
// (c) aichingert

fn update(cur: &mut Vec<char>) -> bool {
    let len = cur.len()-1;
    for i in 0..cur.len() {
        match cur[len-i] {
            'z' => cur[len-i] = 'a',
            _ => cur[len-i] = ((cur[len-i] as u8) + 1u8) as char,
        }

        if cur[len-i] != 'a' {
            break;
        }
    }

    if cur[0] == 'l' || cur[0] == 'o' || cur[0] == 'i' {
        return false;
    }

    let mut trip: bool = false;
    let mut amt: u8 = 0;
    let mut i = 0usize;

    for i in 0..cur.len()-2 {
        if (cur[i] as u8) + 1u8 == (cur[i+1] as u8) && (cur[i+1] as u8) + 1u8 == (cur[i+2] as u8) {
            trip = true;
            break;
        }
    }

    while i < cur.len()-1 {
        if cur[i+1] == 'l' || cur[i+1] == 'o' || cur[i+1] == 'i' {
            return false;
        }
        if cur[i] == cur[i+1] {
            amt += 1;
            i += 1;
        }
        i += 1;
    }

    trip && amt > 1
}

fn solve(cur: &mut Vec<char>) -> String {
    while !update(cur) {}
    cur.iter().map(|ch| *ch).collect::<String>()
}

fn main() {
	let inp = std::fs::read_to_string("../input/11").unwrap();
    let mut inp = inp.trim().chars().collect::<Vec<char>>();

    println!("Part 1: {}", solve(&mut inp));
    println!("Part 2: {}", solve(&mut inp));
}
