fn main() {
    let inp = std::fs::read_to_string("../input/01").unwrap().trim().to_string();
    let inp = inp.lines().collect::<Vec<_>>();

    let mut part_one: u32 = 0;
    
    for i in 0..inp.len() - 1 {
        if inp[i+1] > inp[i] {
            part_one += 1;
        }
    }



    println!("Part one: {}", part_one);
}
