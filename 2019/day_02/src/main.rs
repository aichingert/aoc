fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
    solve_part_two(&input);
}

fn solve_part_one(input: &String) {
    let mut i: Vec<usize> = input
        .split(',')
        .map( | c | c.parse::<usize>().unwrap())
        .collect();

    let mut idx: usize = 0;

    loop {
        let inst: usize = i[idx];

        if inst == 99 {
            break;
        }

        let f: usize = i[idx + 1];
        let s: usize = i[idx + 2];
        let t: usize = i[idx + 3];

        match inst {
            1 => {
                i[t] = i[f] + i[s]
            },
            2 => {
                i[t] = i[f] * i[s]
            },
            _ => {}
        }

        idx += 4;
    }

    println!("Solution part one: {}", i[0]);
}

fn solve_part_two(input: &String) {
    let mut noun: usize = 0;
    let mut verb: usize = 0;

    'main: for n in 0..100 {
        for v in 0..100 {
            let mut i: Vec<usize> = input
                .split(',')
                .map( | c | c.parse::<usize>().unwrap())
                .collect();
            
            i[1] = n;
            i[2] = v;
    
            let mut idx: usize = 0;
        
            loop {
                let inst: usize = i[idx];
        
                if inst == 99 {
                    break;
                }
        
                let f: usize = i[idx + 1];
                let s: usize = i[idx + 2];
                let t: usize = i[idx + 3];
        
                match inst {
                    1 => {
                        i[t] = i[f] + i[s]
                    },
                    2 => {
                        i[t] = i[f] * i[s]
                    },
                    _ => {}
                }
        
                idx += 4;
            }

            if i[0] == 19690720 {
                noun = n;
                verb = v;
                break 'main;
            }
        }
    }

    println!("Solution part two: {}", 100 * noun + verb);
}