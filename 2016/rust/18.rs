// Advent of Code 2016, day 18
// (c) aichingert

fn solve(start: &mut Vec<Vec<char>>, steps: usize) -> usize {
    for gen in start.len()-1..steps-1 {
        start.push(vec![]);
        for loc in 0..start[gen].len() {
            let left = loc>0 && start[gen][loc-1] == '^';
            let right = loc+1<start[gen].len() && start[gen][loc+1] == '^';

            if (left && !right) || (!left && right) || (left && !right) || (!left && right) {
                start[gen+1].push('^');
            } else {
                start[gen+1].push('.');
            }
        }
    }

    start.iter().map(|vec| vec.iter().filter(|&trap| *trap != '^').count()).sum::<usize>()
}

fn main() {
    let mut inp = vec![std::fs::read_to_string("../input/18").unwrap().trim().chars().collect::<Vec<char>>()];

    println!("Part 1: {}", solve(&mut inp, 40));
    println!("Part 2: {}", solve(&mut inp, 400000));
}
