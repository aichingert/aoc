pub fn solve() {
    let mut map = std::fs::read_to_string("input/2021/25").unwrap().trim().lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut steps = 1;

    loop {
        let mut moved = false;
        let mut next = vec![vec!['.'; map[0].len()]; map.len()];

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                match map[i][j] {
                    '>' => {
                        let mut right = (j + 1) % map[i].len();

                        if map[i][right] == '.' {
                            moved = true;
                        } else { 
                            right = j; 
                        }
                        
                        next[i][right] = '>';
                    }
                    'v' => {
                        let mut down = (i + 1) % map.len();
                        let left = (j as i32 - 1).rem_euclid(map[i].len() as i32) as usize;
                        let right = (j + 1) % map[i].len();
                    
                        if (map[down][right] == '.' || map[down][j] != '>') 
                        && (map[down][left] != '>' || (map[down][j] == '>' && map[down][right] == '.'))
                        && map[down][j] != 'v' {
                            moved = true;
                        } else { 
                            down = i; 
                        }
                        
                        next[down][j] = 'v';
                    }
                    '.' => (),
                    c => panic!("Invalid case: expected '>' or 'v' got -> {}", c),
                }
            }
        }

        if !moved {
            break;
        }

        steps += 1;
        map = next;
    }

    println!("Part one: {steps:?}");
}
