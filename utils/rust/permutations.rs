// Permutations

use std::fmt::Display;

// [A,B,C] -> [A,B,C], [B,A,C], [C,A,B], [B,C,A], [C,B,A], [A,C,B]
pub fn permutations<T: Display + Clone>(size: usize, vec: &mut Vec<T>) -> Vec<Vec<T>> {
    let mut perms = Vec::<Vec<T>>::new();
    let mut c = vec![0;size];

    perms.push(vec.clone());
    let mut i = 0usize;

    while i < size {
        if c[i] < i {
            if i % 2 == 0 {
                vec.swap(0, i);
            } else {
                vec.swap(c[i], i);
            }

            perms.push(vec.clone());
            c[i] += 1;
            i = 1;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    perms
}
