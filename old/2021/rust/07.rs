fn solve<F>(c: &Vec<i32>, pred: F, l: i32, h: i32) -> i32 
    where F: Fn(i32, i32) -> i32 {
    (l..=h)
        .map(|i| c.iter().map(|d| pred(*d, i)).sum())
        .min()
        .unwrap()
}

fn main() {
    let inp = std::fs::read_to_string("../input/07").unwrap().trim().to_string();
    let inp = inp.split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let (min, max) = (*inp.iter().min().unwrap(), *inp.iter().max().unwrap());

    println!("Part one: {}", solve(&inp, |d: i32, i: i32| -> i32 { (d - i).abs() }, min, max));
    println!("Part two: {}", solve(&inp,|d:i32,i:i32|->i32 {let a=(d-i).abs();(a*(a+1))/2},min,max));
}
