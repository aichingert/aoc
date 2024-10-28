use md5;

fn main() {
    let inp = std::fs::read_to_string("../../../input/2015/04").unwrap().trim().to_string();
    let mut p = 0;

    for i in 1.. {
        let b = format!("{inp}{i}").chars().map(|c| c as u8).collect::<Vec<_>>();
        let d = md5::compute(b);

        if p == 0 && d[0] == 0 && d[1] == 0 && d[2] & 0xF0 == 0 ||
            p == 1 && d[0] == 0 && d[1] == 0 && d[2] == 0 {
            p += 1;
            println!("Part {p}: {i}");

            if p == 2 {
                break;
            }
        }
    }

}
