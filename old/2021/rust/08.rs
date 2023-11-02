fn part_one(inp: &Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    inp
        .iter()
        .map(|(_, out)| out.iter().filter(|o| matches!(o.len(), 2 | 3 | 4 | 7)).count())
        .sum::<usize>()
}

fn get_easy<'a>(line: &Vec<&'a str>) -> (&'a str, &'a str, &'a str, &'a str, Vec<usize>, Vec<usize>) {
    let (mut one, mut seven, mut four, mut eight) = (0, 0, 0, 0);
    let mut zero_six_or_nine = Vec::new();
    let mut two_three_or_five = Vec::new();

    for i in 0..line.len() {
        match line[i].len() {
            2 => one = i,
            3 => seven = i,
            4 => four = i,
            5 => two_three_or_five.push(i),
            6 => zero_six_or_nine.push(i),
            7 => eight = i,
            _ => (),
        }
    }

    (line[one], line[seven], line[four], line[eight], two_three_or_five, zero_six_or_nine)
}

fn get_six_and_lower_one<'a>(l: &Vec<&'a str>, o: &'a str, z: &Vec<usize>) -> (usize, usize) {
    for i in 0..z.len() {
        let mut lower = 0;
        let mut found = 0;

        for j in 0..o.len() {
            if l[z[i]].contains(&o[j..j+1]) {
                lower = j;
                found += 1;
            }
        }

        if found == 1 {
            return (i, lower);
        }
    }

    panic!("invalid segment");
}

fn get_two_three_and_five<'a>(line: &Vec<&'a str>, one: &'a str, lower_one:usize, ttf: &Vec<usize>)->(&'a str,&'a str,&'a str) {
    let (mut two, mut three, mut five) = (0, 0, 0);
    let upper_one = if lower_one == 0 { 1 } else { 0 };

    for i in 0..ttf.len() {
        match (line[ttf[i]].contains(&one[upper_one..upper_one+1]), line[ttf[i]].contains(&one[lower_one..lower_one+1])) {
            (true, false) => two = ttf[i],
            (true, true)  => three = ttf[i],
            (false, true) => five = ttf[i],
            _ => panic!("invalid segment"),
        }
    }

    (line[two], line[three], line[five])
}

fn get_zero_and_nine<'a>(l: &Vec<&'a str>, o: &'a str, tw: &'a str, th: &'a str, sn:&Vec<usize>) -> (&'a str,&'a str) {
    let mut segment = 0;

    for i in 0..tw.len() {
        if !(o.contains(&tw[i..i+1]) || th.contains(&tw[i..i+1])) {
            segment = i;
        }
    }

    if l[sn[0]].contains(&tw[segment..segment+1]) {
        (l[sn[0]], l[sn[1]])
    } else {
        (l[sn[1]], l[sn[0]])
    }
}

fn get_bit_mask(s: &str) -> u8 {
    let mut mask = 0b0;

    for ch in s.chars() {
        mask |= 1 << (ch as u8 - b'a');
    }

    mask
}

fn part_two(inp: &Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
    inp.iter().map(|line| {
        let (one, seven, four, eight, ttf, mut zsn) = get_easy(&line.0);
        let (six, lower_one) = get_six_and_lower_one(&line.0, one, &zsn);
        let (two, three, five) = get_two_three_and_five(&line.0, one, lower_one, &ttf);
        let six = line.0[zsn.remove(six)];
        let (zero, nine) = get_zero_and_nine(&line.0, one, two, three, &zsn);

        let segments = vec![zero, one, two, three, four, five, six, seven, eight, nine]
            .iter()
            .map(|d| get_bit_mask(d))
            .collect::<Vec<_>>();

        let mut nums = Vec::new();
        for i in 0..line.1.len() {
            nums.extend_from_slice(&(0..segments.len())
                .filter(|j| segments[*j] == get_bit_mask(line.1[i]))
                .map(|j| j)
                .collect::<Vec<usize>>());
        }

        nums.iter().fold(0, |acc, cur| acc * 10 + cur) as u32
    }).sum::<u32>()
}

fn main() {
    let inp = std::fs::read_to_string("../input/08").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| {
        let (p, o) = l.split_once(" | ").unwrap();
        (p.split(' ').collect::<Vec<_>>(), o.split(' ').collect::<Vec<_>>())
    }).collect::<Vec<_>>();

    println!("Part one: {}", part_one(&inp));
    println!("Part two: {}", part_two(&inp));
}
