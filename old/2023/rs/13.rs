fn main() {
    let inp = std::fs::read_to_string("../input/13").unwrap().trim().to_string();
    let inp = inp.split("\n\n").collect::<Vec<_>>();

    let mut ans = 0;

    for pattern in inp {
        let map = pattern.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

        let mut split = (map[0].len() as f32 / 2.0).ceil() as usize;
        let mut rows = 0;
        let mut cols = 0;

        for i in 0..split {
            let mut row = Vec::new();
            let mut mir = Vec::new();

            let m_idx = split * 2 - i - 1;

            if m_idx < map[0].len() {
                for j in 0..map.len() {
                    row.push(map[j][i]);
                    mir.push(map[j][m_idx]);
                }
            }


            if row != mir {
                break;
            }

            rows += 1;
        }

        split = (map.len() as f32 / 2.0).ceil() as usize;

        for i in 0..split {
            let mut col = Vec::new();
            let mut mir = Vec::new();

            let m_idx = split * 2 - i - 1;

            if m_idx < map.len() {
                for j in 0..map[0].len() {
                    col.push(map[i][j]);
                    mir.push(map[m_idx][j]);
                }
            }

            if col != mir {
                break;
            }
            cols += 1;
        }

        if cols >= rows {
            ans += cols * 100;
        } else {
            ans += rows;
        }
    }
    println!("{ans}");
}
