const SIZE: usize = 256;

fn main() {
    let content = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&content);
    solve_part_two(content);
}

fn solve_part_one(content: &String) {
    let mut elements: [i32; SIZE] = (0..256).collect::<Vec<_>>()
    .try_into().expect("wrong size iterator");
    let mut min: usize = 0;
    let mut range: usize = 0;
    let mut skip_size: usize = 0;
    let mut changing: usize = 0;

    let numbers: Vec<_> = content.split(',').map( | str | {
        str.parse::<usize>().unwrap()
    }).collect();

    for number in numbers {
        min = changing;

        changing += skip_size + number;
        skip_size += 1;

        changing %= SIZE;

        range = number;

        let cut = reverse(&mut min, &mut range, &mut elements);

        let mut min_idx = min;

        for i in 0..cut.len() {
            if min_idx >= elements.len() {
                min_idx = 0;
            }

            elements[min_idx] = cut[i];
            min_idx += 1;
        }
    }

    println!("Solution part 1: {}", elements[0] * elements[1]);
}

fn solve_part_two(content: String) {
    let mut elements: [i32; SIZE] = (0..256).collect::<Vec<_>>()
    .try_into().expect("wrong size iterator");
    let mut min: usize = 0;
    let mut range: usize = 0;
    let mut skip_size: usize = 0;
    let mut changing: usize = 0;

    let mut numbers: Vec<_> = content.chars().map( | str | {
        str as i32
    }).collect();
    let prefixes: Vec<i32> = vec![17, 31, 73, 47, 23];

    for prefix in prefixes {
        numbers.push(prefix)
    }

    println!("{:?}", numbers)
}

fn reverse(min: &mut usize, number_range: &mut usize, elements: &mut [i32; SIZE]) -> Vec<i32> {
    let mut min_idx: usize = *min;
    let mut cut: Vec<i32> = Vec::new();

    for i in 0..*number_range {
        if min_idx >= elements.len() {
            min_idx = 0;
        }

        cut.push(elements[min_idx].clone());
        min_idx += 1;
    }

    let len = cut.len();

    for i in 0..cut.len()/2 {
        let swap = cut[i];
        cut[i] = cut[len-i-1];
        cut[len-i-1] = swap;
    }

    cut
}

/*      min = skip_size;
        skip_size = max;
        max = skip_size + number;

        max %= SIZE;


        println!("{} ", skip_size);
        
        let cut = reverse(&mut min, &mut max, &mut elements);

        let mut min_idx = min;

        for i in 0..cut.len() {
            if min_idx >= elements.len() {
                min_idx = 0;
            }

            elements[min_idx] = cut[i];
            min_idx += 1;
        }

        skip_size += 1;
        skip_size %= SIZE; 
*/