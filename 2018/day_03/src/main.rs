fn main() {
    let input: Box::<String> = Box::new(std::fs::read_to_string("input.txt").expect("err"));

    solve_part_one(&input);
}

fn solve_part_one(input: &String) {
    let mut fabric: Box<Vec<Vec<(usize, u16)>>> = Box::new(vec![vec![(0, 20); 1000]; 1000]);

    for line in input.lines() {
        let commands: Vec<_> = line.split(" @ ").collect();

        let cords: Vec<_> = commands[1].split(": ").collect();

        let position: Vec<_> = cords[0].split(',').collect();
        let size: Vec<_> = cords[1].split('x').collect();

        let x: usize = position[0].parse::<usize>().unwrap();
        let y: usize = position[1].parse::<usize>().unwrap();

        for i in 0..size[1].parse::<usize>().unwrap() {
            for j in 0..size[0].parse::<usize>().unwrap() {
                fabric[y + i][x + j].0 += 1;
                fabric[y + i][x + j].1 = commands[0].replace("#", "").parse::<u16>().unwrap();
            }
        }
    }

    let mut count: usize = 0;

    for i in 0..fabric.len() {
        for j in 0..fabric[i].len() {
            if fabric[i][j].0 > 1 {
                count += 1;
            }
        }
    }

    println!("Solution part one: {}", count);
    solve_part_two(&fabric, input);
}

fn solve_part_two(fabric: &Box<Vec<Vec<(usize, u16)>>>, input: &String) {
    let mut not_overlapping: bool;
    let mut id: u16 = 0;

    for line in input.lines() {
        not_overlapping = true;
        let commands: Vec<_> = line.split(" @ ").collect();

        let cords: Vec<_> = commands[1].split(": ").collect();

        let position: Vec<_> = cords[0].split(',').collect();
        let size: Vec<_> = cords[1].split('x').collect();

        let x: usize = position[0].parse::<usize>().unwrap();
        let y: usize = position[1].parse::<usize>().unwrap();

        for i in 0..size[1].parse::<usize>().unwrap() {
            for j in 0..size[0].parse::<usize>().unwrap() {
                if fabric[y + i][x + j].0 != 1 { 
                    not_overlapping = false;
                }
            }
        }

        if not_overlapping {
            id = fabric[y][x].1;
        }
    }

    println!("Solution part two: {}", id);
}