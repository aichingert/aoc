fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
}

fn solve_part_one(input: &String) {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 50]; 6];

    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();
        match split[0] {
            "rect" => {
                let sizes: Vec<usize> = split[1].split('x').map( | x | x.parse::<usize>().unwrap() ).collect();
                
                for i in 0..sizes[1] {
                    for j in 0..sizes[0] {
                        grid[i][j] = '#';
                    }
                }
            },
            "rotate" => {
                let pos: Vec<&str> = split[2].split('=').collect(); // 4
                let amount: usize = split[4].parse::<usize>().unwrap();

                let mut rc: Vec<char> = Vec::new();

                match split[1] {
                    "row" => {
                        for _ in 0..amount {
                            let cur: char = grid[pos[1].parse::<usize>().unwrap()].pop().unwrap();
                            grid[pos[1].parse::<usize>().unwrap()].insert(0, cur);
                        }
                    },
                    _ => {
                        for i in 0..grid.len() {
                            rc.push(grid[i][pos[1].parse::<usize>().unwrap()]);
                        }

                        for _ in 0..amount {
                            let cur: char = rc.pop().unwrap();
                            rc.insert(0, cur);
                        }

                        for i in 0..grid.len() {
                            grid[i][pos[1].parse::<usize>().unwrap()] = rc[i];
                        }
                    }
                }
            },
            _ => {}
        }
    }

    println!("Solution part one: {}", count(&grid));
    solve_part_two(&grid);
}

fn solve_part_two(grid: &Vec<Vec<char>>) {
    println!("Solution part two: ");

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != '.' {
                print!("{}", grid[i][j]);
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn count(grid: &Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '#' {
                count += 1;
            }
        }
    }

    count
}