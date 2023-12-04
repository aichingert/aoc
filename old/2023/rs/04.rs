use std::collections::HashSet;

fn main() {
    let inp = std::fs::read_to_string("../input/04").unwrap().trim().to_string();
    let mut ans = 0;
    let mut vec = vec![1; inp.lines().collect::<Vec<_>>().len()];

    for (i, line) in inp.lines().enumerate() {
        let (_, n) = line.split_once(": ").unwrap();
        let (w, m) = n.split_once(" | ").unwrap();

        let (w, m) = (w.split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>(), m.split(' ').filter(|s| !s.is_empty()).collect::<Vec<_>>());

        let mut n = 0usize;

        for i in 0..m.len() {
            if w.contains(&m[i]) {
                n += 1;
            }
        }

        for j in 1..=n {
            vec[i + j] += vec[i];
        }
    }

    println!("{:?}", vec);
    println!("{}", vec.iter().sum::<u32>());
}
