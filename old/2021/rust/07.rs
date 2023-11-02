fn solve<F>(c: &Vec<i32>, pred: F, l: i32, h: i32) -> i32 
    where F: Fn(i32, i32) -> i32 {
    let mut ans = i32::MAX;

    for i in l..=h {
        ans = ans.min(c.iter().map(|d| pred(*d, i)).sum());
    }

    ans
}

fn main() {
    let inp = std::fs::read_to_string("../input/07").unwrap().trim().to_string();
    let inp = inp.split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let min = *inp.iter().min().unwrap();
    let max = *inp.iter().max().unwrap();

    println!("Part one: {}", solve(&inp, |d: i32, i: i32| -> i32 { (d - i).abs() }, min, max));
    println!("Part two: {}", solve(&inp,|d:i32,i:i32|->i32 {let a=(d-i).abs();(a*(a+1))/2},min,max));
}
