pub fn solve() {
    let points = std::fs::read_to_string("input/2018/25")
        .unwrap()
        .trim()
        .lines()
        .map(|l| {
            l.split(',')
                .map(|n| n.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut constellations = Vec::<Vec<Vec<i32>>>::new();

    for point in points {
        let mut near = Vec::new();

        'outer: for i in 0..constellations.len() {
            for co_point in constellations[i].iter() {
                let dist = point
                    .iter()
                    .zip(co_point.iter())
                    .map(|(c1, c2)| (c2 - c1).abs())
                    .fold(0, |a, x| a + x);

                if dist <= 3 {
                    near.push(i);
                    continue 'outer;
                }
            }
        }

        match near.len() {
            0 => constellations.push(vec![point]),
            1 => constellations[near[0]].push(point),
            _ => {
                let mut mix = vec![point];
                for i in 0..near.len() {
                    mix.extend_from_slice(&constellations.remove(near[i] - i));
                }
                constellations.push(mix);
            }
        }
    }

    println!("Part one: {}", constellations.len());
    println!("Part two: :)");
}
