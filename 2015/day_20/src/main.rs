const INPUT: usize = 33100000;

fn main() {
    solve_part_one();
    solve_part_two(11, 50);
}

fn solve_part_one() {
    let mut house_number: usize = 0;

    for i in 1..INPUT {
        let mut current: usize = 0;

        for j in 1..=i {
            if i % j == 0 {
                current += 10 * j;
            }
        }

        if current >= INPUT {
            house_number = i;
            break;
        }
    }

    println!("Solution part one: {}", house_number);
}
    
fn solve_part_two(presents_per_elf: usize, houses_per_elf: usize) -> usize {
    let num_houses = 1000 * 1000 * 10;
    let mut houses: Vec<usize> = vec![0; num_houses];

    for i in 1..num_houses {
        let steps = std::cmp::min(num_houses / i, houses_per_elf);
        for quot in 1..steps {
            let idx = quot * i;
            houses[idx] += i * presents_per_elf;
        }
        if houses[i] >= INPUT {
            println!("Solution part two: {}", i);
            return i;
        }
    }

    return 0;
}