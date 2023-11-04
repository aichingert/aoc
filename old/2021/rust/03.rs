fn count_zeros_and_ones(pos: usize, inp: &Vec<Vec<char>>) -> (u32, u32) {
    let (mut zeros, mut ones) = (0, 0);

    for i in 0..inp.len() {
        match inp[i][pos] {
            '0' => zeros += 1,
            '1' => ones  += 1,
            _ => (),
        }
    }

    (zeros, ones)
}

fn part_one(inp: &Vec<Vec<char>>) -> u32 {
    let (mut gamma, mut filter, len) = (0u32, 0u32, inp[0].len());

    for i in 0..len {
        let (zeros, ones) = count_zeros_and_ones(i, inp);

        if ones > zeros {
            gamma |= 1 << (len - i - 1);
        }

        filter |= 1 << i;
    }

    gamma * (!gamma & filter)
}

fn part_two(mut inp: Vec<Vec<char>>) -> u32 {
    let len = inp[0].len();
    let mut cp = inp.clone();

    for i in 0..len {
        let (zeros, ones) = count_zeros_and_ones(i, &cp);
        let predicate = if ones >= zeros { '1' } else { '0' };

        cp = cp.into_iter().filter(|line| line[i] == predicate).collect::<Vec<_>>();

        if inp.len() > 1 {
            let (zeros, ones) = count_zeros_and_ones(i, &inp);
            let predicate = if zeros <= ones { '0' } else { '1' };

            inp = inp.into_iter().filter(|line| line[i] == predicate).collect::<Vec<_>>();
        }
    }

    let (mut oxygen_generator_rating, mut co2_scrubber_rating) = (0, 0);

    for i in 0..len {
        if cp[0][i] == '1' {
            oxygen_generator_rating |= 1 << (len - i - 1);
        }
        if inp[0][i] == '1' {
            co2_scrubber_rating |= 1 << (len - i - 1);
        }
    }

    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let inp = std::fs::read_to_string("../input/03").unwrap().trim().to_string();
    let inp = inp.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    println!("Part one: {}", part_one(&inp));
    println!("Part two: {}", part_two(inp));
}
