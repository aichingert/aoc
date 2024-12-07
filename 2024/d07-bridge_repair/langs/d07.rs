fn sol(ex: u64, eq: &[u64], cu: u64, pa: bool) -> bool {
    if eq.is_empty() {
        return ex == cu;
    }

    (if pa { 
        false 
    } 
    else { 
        sol(ex, &eq[1..], format!("{cu}{}", eq[0]).parse::<u64>().unwrap(), pa) 
    })
        || sol(ex, &eq[1..], cu + eq[0], pa) 
        || sol(ex, &eq[1..], cu * eq[0], pa)
}

fn main() {
    let (p1, p2) = std::fs::read_to_string("../../../input/2024/07").unwrap()
        .lines()
        .filter(|s| !s.is_empty())
        .fold((0, 0), |(mut p1, mut p2), l| {
            let (r, eq) = l.split_once(": ").unwrap();
            let (r, eq) = (
                r.parse::<u64>().unwrap(), 
                eq.split(" ").map(|n| n.parse::<u64>().unwrap()).collect::<Vec<_>>()
            );

            if sol(r, &eq, 0, true) { p1 += r; }
            if sol(r, &eq, 0, false) { p2 += r; }

            (p1, p2)
        });

    println!("p1: {p1}");
    println!("p2: {p2}");

}

