fn which(chs: &Vec<Vec<char>>, ch: i32) -> Option<u32> {
    for i in 1..chs.len() {
        let mut rp = i;
        let mut mp = i as i32 - 1;

        while mp >= 0 && rp < chs.len() && chs[rp] == chs[mp as usize] {
            rp += 1;
            mp -= 1;
        }

        if mp < ch && ch < rp as i32 && (rp == chs.len() || mp == -1) {
            return Some(i as u32);
        }
    }

    None
}

fn get_reflection(map: &Vec<Vec<char>>, r: i32, c: i32) -> Option<u32> {
    let mut rows = Vec::<Vec<char>>::new();
    let mut cols = Vec::<Vec<char>>::new();

    for i in 0..map.len() {
        rows.push(map[i].clone());
    }

    if let Some(n) = which(&rows, r) {
        return Some(n * 100);
    }

    for j in 0..map[0].len() {
        let mut col = Vec::new();

        for i in 0..map.len() {
            col.push(map[i][j]);
        }

        cols.push(col);
    }

    which(&cols, c)
}

fn main() {
    let inp = std::fs::read_to_string("../input/13").unwrap().trim().to_string();
    let inp = inp.split("\n\n").collect::<Vec<_>>();

    let mut ans = 0;
    let mut p2ans = 0;

    'lp: for pattern in inp {
        let mut map = pattern.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
        //ans += get_reflection(&map, 0, 0).unwrap();

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                map[i][j] = if map[i][j] == '#' { '.' } else { '#' };

                if let Some(n) = get_reflection(&map, i as i32, j as i32) {
                    p2ans += n;
                    continue 'lp;
                }

                map[i][j] = if map[i][j] == '#' { '.' } else { '#' };
            }
        }

        
    }

    println!("{ans}");
    println!("{p2ans}");
}
