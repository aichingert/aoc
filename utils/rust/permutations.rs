// Permutations

use std::fmt::Display;

// [A,B,C] -> [A,B,C], [B,A,C], [C,A,B], [B,C,A], [C,B,A], [A,C,B]
pub fn permutations<T: Display + Clone>(size: usize, vec: &mut Vec<T>, perms: &mut Vec<Vec<T>>) {
    if size == 1 {
        perms.push(vec.clone());
    } else {
        permutations(size - 1, &mut vec.clone(), perms);

        for i in 0..size-1 {
            if i % 2 == 0 {
                vec.swap(i, size-1)
            } else {
                vec.swap(0, size-1)
            } 
            permutations(size-1, &mut vec.clone(), perms);
        }
    }
}
