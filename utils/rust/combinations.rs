// Advent of Code, Combinations

// A B C 3
// A
// B
// C
// A B
// A C
// B C
// A B C

use std::fmt::Display;

pub fn combinations<T: Display + Clone>(size: usize, vec: &Vec<T>) -> Vec<Vec<T>> {
    let mut comb: Vec<Vec<T>> = Vec::new();

    if size > vec.len() { return comb; }
    let mut idxs = (0..size).collect::<Vec<usize>>();

    loop {
        let mut done = true;

        'inc: for inc in (0..size).rev() {
            idxs[inc] += 1;
            if idxs[inc] == vec.len() {
                continue;
            }
            
            for nxt in inc+1..size {
                idxs[nxt] = idxs[nxt-1]+1;
                if idxs[nxt] == vec.len() {
                    continue 'inc;
                }
            }
            
            done = false;
            comb.push(idxs.iter().map(|i| vec[*i].clone()).collect::<Vec<T>>());
            break;
        }

        if done {
            return comb;
        }
   }
}
