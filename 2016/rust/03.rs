// Advent of Code 2016, day 3
// (c) aichingert

fn valid_triangle(ver: &Vec<i32>) -> bool {
    ver[0] + ver[1] > ver[2] && ver[1] + ver[2] > ver[0] && ver[0] + ver[2] > ver[1]
}

fn part1(inp: &Vec<Vec<i32>>) -> usize {
    inp.iter().filter(|v| valid_triangle(v)).count()
}

fn part2(inp: &Vec<Vec<i32>>) -> usize {
    let mut ans: usize = 0;

    for i in (0..inp.len()).step_by(3) {
        ans += [[inp[i][0],inp[i+1][0],inp[i+2][0]],[inp[i][1],inp[i+1][1],inp[i+2][1]],[inp[i][2],inp[i+1][2],inp[i+2][2]]]
            .iter()
            .filter(|v| valid_triangle(&v.to_vec()))
            .count();
    }

    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/03").unwrap();
    let inp = inp.lines().map(|l| l.split(' ').filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>();

    println!("Part 1: {}", part1(&inp));
    println!("Part 2: {}", part2(&inp));
}
