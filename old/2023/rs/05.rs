fn part_one(seeds: &Vec<u32>, ranges: &Vec<Vec<Vec<u32>>>) -> u32 {
    seeds.iter().map(|n| {
        let mut seed = *n;

        'outer: for funcs in ranges {
            for range in funcs {
                if seed >= range[1] && seed < range[1] + range[2] {
                    seed = range[0] + (seed - range[1]);
                    continue 'outer;
                }
            }
        }

        seed
    }).min().unwrap()
}

fn part_two(seeds: &Vec<u32>, ranges: &Vec<Vec<Vec<u32>>>) -> u32 {
    (0..seeds.len()).step_by(2).map(|i| {
        (seeds[i]..(seeds[i] + seeds[i + 1])).map(|mut seed| {
            'outer: for funcs in ranges {
                for range in funcs {
                    if seed >= range[1] && seed < range[1] + range[2] {
                        seed = range[0] + (seed - range[1]);
                        continue 'outer;
                    }
                }
            }

            seed
        }).min().unwrap()
    }).min().unwrap()
}

fn n<T>(n: &str) -> T
    where T: std::str::FromStr<Err = std::num::ParseIntError>
{
    n.parse::<T>().unwrap()
}

fn parse() -> (Vec<u32>, Vec<Vec<Vec<u32>>>) {
    let inp = std::fs::read_to_string("../input/05").unwrap().trim().to_string();
    let inp = inp.split("\n\n").collect::<Vec<_>>();
    let seeds = inp[0].split_whitespace().skip(1).map(n::<u32>).collect::<Vec<_>>();

    (seeds, (1..inp.len()).map(|i| inp[i].lines().skip(1)
            .map(|l| l.split_whitespace().map(n::<u32>).collect::<Vec<u32>>())
            .collect::<Vec<_>>()).collect::<Vec<_>>())
}

fn main() {
    let (seeds, ranges) = parse();

    println!("Part one: {}", part_one(&seeds, &ranges));
    println!("Part two: {}", part_two(&seeds, &ranges));
}
