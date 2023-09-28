fn main() {
    let inp = std::fs::read_to_string("../input/04").unwrap().trim().to_string();

    let (lower, upper) = inp.split_once("-").unwrap();
    let (lower, upper) = (lower.parse::<u32>().unwrap(), upper.parse::<u32>().unwrap());

    let mut part_one = 0;
    let mut part_two = 0;

    for x in lower..=upper {
        let xs = x.to_string().chars().map(|l| (l as u8 - b'0')).collect::<Vec<u8>>();
        let mut d = false;

        for i in 0..xs.len() - 1 {
            if xs[i] > xs[i + 1] {
                d = false;
                break;
            }

            if xs[i] == xs[i + 1] {
                d = true;
            }
        }

        let mut i = 0;

        // suboptimal
        while i < xs.len() - 1 {
            let mut cur = 1;
            let mut j = i + 1;

            while j < xs.len() {
                if xs[i] == xs[j] {
                    cur += 1;
                    i += 1;
                }

                j+= 1;
            }

            if d && cur == 2 {
                part_two += 1;
                break;
            }
            i += 1;
        }

        if d {
            part_one += 1;
        }
    }

    println!("Part one: {}", part_one);
    println!("Part two: {}", part_two);
}

