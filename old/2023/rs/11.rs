fn main() {
    let inp = std::fs::read_to_string("../input/11").unwrap().trim().to_string();
    let mut inp = inp.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("{:?}", inp);
    let mut stars = Vec::new();
    let (mut ofi, mut ofj) = (0, 0);

    for i in 0..inp.len() {
        ofj = 0;
        let mut cstar = false;

        for j in 0..inp[i].len() {
            let mut rstar = false;
            for k in 0..inp.len() {
                if inp[k][j] == '#' {
                    rstar = true;
                }
            }

            if !rstar {
                ofj += 1000000 - 1;
            }

            if inp[i][j] == '#' {
                stars.push(((i + ofi) as i64, (j + ofj) as i64));
                cstar = true;
            }
        }

        if !cstar {
            ofi += 1000000 - 1;
        }

    }

    let mut ans = 0;

    for i in 0..stars.len() - 1 {
        for j in (i + 1)..stars.len() {
            let dist = (stars[i].0 - stars[j].0).abs() + (stars[i].1 - stars[j].1).abs();
            println!("{:?} - {:?} -> {:?}", stars[i], stars[j], dist);
            ans += dist;
        }
        //ans += dist;
    }

    println!("{}", ans);
}
