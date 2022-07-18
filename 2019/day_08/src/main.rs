const X: i32 = 25;
const Y: i32 = 6;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input.trim().to_string());
    solve_part_two(&input.trim().to_string());
}

fn solve_part_one(input: &String) {
    let mut layers: Vec<Vec<i32>> = Vec::new();
    let numbers: Vec<char> = input.chars().collect();

    let mut i: usize = 0;

    while i < numbers.len() {
        let mut layer: Vec<i32> = Vec::new();

        for j in 0..X*Y {
            layer.push((numbers[i + j as usize] as u8 - '0' as u8) as i32);
        }

        layers.push(layer);
        i += (X*Y) as usize;
    }

    drop(i);

    let mut zero_count: i32 = layers[0].len() as i32;
    let mut one_count: i32 = 0;
    let mut two_count: i32 = 0;

    for i in 0..layers.len() {
        let mut z_count: i32 = 0;
        let mut o_count: i32 = 0;
        let mut t_count: i32 = 0;

        for j in 0..layers[i].len() {
            match layers[i][j] {
                0 => {
                    z_count += 1;
                },
                1 => {
                    o_count += 1;
                },
                2 => {
                    t_count += 1;
                }
                _ => {}
            }
        }

        if z_count < zero_count {
            zero_count = z_count;
            one_count = o_count;
            two_count = t_count;
        }
    }

    println!("Solution part one: {:?}", one_count * two_count);
}

fn solve_part_two(input: &String) {
    let mut layers: Vec<Vec<i32>> = Vec::new();
    let numbers: Vec<char> = input.chars().collect();
    let mut resulting: Vec<i32> = Vec::new();

    let mut i: usize = 0;

    while i < numbers.len() {
        let mut layer: Vec<i32> = Vec::new();

        for j in 0..X*Y {
            layer.push((numbers[i + j as usize] as u8 - '0' as u8) as i32);
        }

        layers.push(layer);
        i += (X*Y) as usize;
    }

    drop(i);

    for i in 0..layers.len() {
        for j in 0..layers[i].len() {
            if i == 0 {
                resulting.push(layers[i][j]);
            } else {
                if layers[i][j] != 2 && resulting[j] == 2 {
                    resulting[j] = layers[i][j];
                }
            }
        }
    }

    println!("Solution part two: ");
    for i in 0..resulting.len() {
        if i % X as usize == 0 {
            println!();
        }

        if resulting[i] == 0 {
            print!(" ");
        } else {
            print!("#");
        }
    }
}