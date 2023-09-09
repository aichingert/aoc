// Advent of Code 2017, day 4
// (c) aichingert

use std::collections::HashSet;

fn filter<T: std::fmt::Debug>(l: &[T]) -> bool 
where T: Eq, T: std::hash::Hash {
    let mut hs = HashSet::<&T>::new();

    for sp in l {
        if !hs.insert(sp) {
            return false;
        }
    }

    true
}

fn solve(inp: &[&str], part: bool) -> usize {
    inp.iter().filter(|l| {
        if part {
            filter::<Vec<char>>(&l.split(' ').map(|sp| { let mut ch = sp.chars().collect::<Vec<char>>(); ch.sort(); ch}).collect::<Vec<Vec<char>>>())
        } else {
            filter::<&str>(&l.split(' ').collect::<Vec<&str>>())
        }
    }).count()
}

fn main() {
    let inp = std::fs::read_to_string("../input/04").unwrap();
    let inp = inp.lines().collect::<Vec<&str>>();

    println!("Part 1: {}", solve(&inp, false));
    println!("Part 2: {}", solve(&inp, true));
}
