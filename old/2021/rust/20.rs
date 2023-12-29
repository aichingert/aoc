fn enhance(map: &Vec<Vec<u16>>, buf: &Vec<u16>) -> Vec<Vec<u16>> {
    let mut next = Vec::new();

    for i in 0..map.len() {
        next.push(vec![]);
        for j in 0..map[i].len() {
            let mut ptr = 0u16;

            for k in -1..2 {
                for l in -1..2 {
                    let (y, x) = (i as i32 + k, j as i32 + l);

                    if y < 0 || x < 0 || y >= map.len() as i32 || x >= map[0].len() as i32 {
                        //ptr |= 1 << (3 * (k + 1) + l + 1);
                    } else {
                        ptr |= map[y as usize][x as usize] << (8 - (3 * (k + 1) + l + 1));
                    }
                }
            }

            next[i].push(buf[ptr as usize]);
        }
    }

    next
}

fn grow(map: &mut Vec<Vec<u16>>) {
    for i in 0..map.len() {
        map[i].insert(0, 0);
        map[i].push(0);
    }

    map.insert(0, vec![0; map[0].len()]);
    map.push(vec![0; map[0].len()]);
}

// 5, 6

fn main() {
    let inp = std::fs::read_to_string("../input/20").unwrap().trim().to_string();
    let (buf, map) = inp.split_once("\n\n").unwrap();

    let buf = buf.chars().map(|c| if c == '#' { 1 } else { 0 }).collect::<Vec<_>>();
    let mut map = map.lines()
        .map(|l| l.chars().map(|c| if c == '#' { 1 } else { 0 }).collect::<Vec<_>>())
        .collect::<Vec<_>>();
 
    for i in 0..2 { 
        grow(&mut map);
        map = enhance(&map, &buf);

        println!("=============================");
        println!("=                           =");
        println!("=             {i}             =");
        println!("=                           =");
        println!("=============================");

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        } 
    }

    for i

}
