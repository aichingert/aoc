fn parse(vec: &mut Vec<Vec<u32>>, line: &str) {
    for l in line.lines().skip(1) {
        let v = l.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<_>>();
        vec.push(v);
    }
}

fn main() {
    let inp = std::fs::read_to_string("../input/05").unwrap().trim().to_string();
    let inp = inp.split("\n\n").collect::<Vec<_>>();

    let mut vecs = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(),Vec::new()];

    parse(&mut vecs[0], inp[1]);
    parse(&mut vecs[1], inp[2]);
    parse(&mut vecs[2], inp[3]);
    parse(&mut vecs[3], inp[4]);
    parse(&mut vecs[4], inp[5]);
    parse(&mut vecs[5], inp[6]);
    parse(&mut vecs[6], inp[7]);

    let seeds = inp[0].split_once(": ").unwrap().1;
    let seeds = seeds.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    let mut ans = u32::MAX;

    for k in (0..seeds.len()).step_by(2) {
        for seed in seeds[k]..seeds[k] + seeds[k + 1] {
            let mut seed = seed;

            for i in 0..vecs.len() {
                for j in 0..vecs[i].len() {
                    if seed >= vecs[i][j][1] && seed < vecs[i][j][1] + vecs[i][j][2] {
                        seed = vecs[i][j][0] + (seed - vecs[i][j][1]);
                        break;
                    }
                }
            }

            ans = seed.min(ans);

        }
    }

    println!("{ans}");
    for seed in seeds {
        let mut seed = seed;

        for i in 0..vecs.len() {
            for j in 0..vecs[i].len() {
                if seed >= vecs[i][j][1] && seed < vecs[i][j][1] + vecs[i][j][2] {
                    seed = vecs[i][j][0] + (seed - vecs[i][j][1]);
                    break;
                }
            }
        }

        ans = seed.min(ans);
    }


}
