const SUBJECTIVE: u64 = 7;

fn solve(inp: (&str, &str)) -> u64 {
    let (card, door): (u64, u64) = (inp.0.parse().unwrap(), inp.1.parse().unwrap());
    let mut dl = 0;
    let mut value = 1;

    loop {
        value = (value * SUBJECTIVE) % 20201227;

        dl += 1;
        if value == door {
            break;
        }
    }

    value = 1;
    for _ in 0..dl {
        value = (value * card) % 20201227;
    }

    value
}

fn main() {
    let inp = std::fs::read_to_string("../input/25").unwrap();
    let inp = inp.split_once('\n').unwrap();

    println!("Part 1: {}", solve(inp));
    println!("Part 2: Merry christmas!");
}
