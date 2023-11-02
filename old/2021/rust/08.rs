fn part_one(inp: &Vec<(Vec<&str>, Vec<&str>)>) -> usize {
    inp
        .iter()
        .map(|(_, out)| out.iter().filter(|o| matches!(o.len(), 2 | 3 | 4 | 7)).count())
        .sum::<usize>()
}

fn get_easy<'a>(line: &Vec<&'a str>) -> (&'a str, &'a str, &'a str, &'a str) {
    let (mut one, mut seven, mut four, mut eight) = (0, 0, 0, 0);

    for i in 0..line.len() {
        match line[i].len() {
            2 => one = i,
            3 => seven = i,
            4 => four = i,
            8 => eight = i,
            _ => (),
        }
    }

    (line[one], line[seven], line[four], line[eight])
}


fn part_two(inp: &Vec<(Vec<&str>, Vec<&str>)>) -> u32 {
    let mut sum = 0u32;

    for line in inp.iter() {
        let (one, seven, four, eight) = get_easy(&line.0);
    }

    sum
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
