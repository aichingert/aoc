use crate::day::{Input, InputError, InputResult, Output, Wrapper};

const Y: i64 = 2000000;
const BOUNDS: i64 = 4000000;

pub struct Sensor {
    x: i64,
    y: i64,
    dist: i64,
}

impl Sensor {
    fn new(dist: i64, x: i64, y: i64) -> Self {
        Sensor { dist, x, y }
    }

    fn dist(x1: i64, x2: i64, y1: i64, y2: i64) -> i64 {
        (x1 - x2).abs() + (y1 - y2).abs()
    }

    fn in_range(sensors: &Vec<Sensor>, x: &mut i64, y: i64) -> bool {
        for sensor in sensors {
            if Sensor::dist(sensor.x, *x, sensor.y, y) <= sensor.dist {
                *x = sensor.x + sensor.dist - (sensor.y - y).abs();
                return true;
            }
        }

        false
    }
}

fn part_one(sensors: &Vec<Sensor>, x: (i64, i64)) -> Output {
    let mut y_count: i64 = 0;
    let mut cur = x.0;

    while cur <= x.1 {
        let cx = cur;

        if Sensor::in_range(sensors, &mut cur, Y) {
            y_count += cur - cx + 1;
        }
        cur += 1;
    }

    Output::Ni64(y_count - 1)
}

fn part_two(sensors: &Vec<Sensor>, beacons: &Vec<(i64, i64)>) -> Output {
    let mut i: i64 = 0;
    let mut j: i64 = 0;

    'outer: while i < BOUNDS {
        j = 0;
        while j < BOUNDS {
            if !Sensor::in_range(sensors, &mut j, i) && !beacons.contains(&(j, i)) {
                break 'outer;
            }
            j += 1;
        }
        i += 1;
    }

    Output::Ni64(j * BOUNDS + i)
}

pub fn run(input: Input) -> (Output, Output) {
    let (sensors, beacons, x): (Vec<Sensor>, Vec<(i64, i64)>, (i64, i64)) = input.unwrap();

    (part_one(&sensors, x), part_two(&sensors, &beacons))
}

pub fn parse() -> InputResult<Input> {
    let (mut sensors, mut beacons) = (Vec::<Sensor>::new(), Vec::<(i64, i64)>::new());
    let (mut min_x, mut max_x) = (i64::MAX, -i64::MAX);

    for line in std::fs::read_to_string("../input/15")?.lines() {
        if let Some((sensor, beacon)) = line.split_once(": ") {
            let elements = vec![
                sensor.split('=').collect::<Vec<&str>>(),
                beacon.split('=').collect::<Vec<&str>>(),
            ];
            let mut cords: Vec<(i64, i64)> = Vec::new();

            for i in 0..elements.len() {
                cords.push(if let Some((x, _)) = elements[i][1].split_once(',') {
                    (x.parse()?, elements[i][2].parse()?)
                } else {
                    return Err(InputError::InvalidInput);
                })
            }

            sensors.push(Sensor::new(
                Sensor::dist(cords[0].0, cords[1].0, cords[0].1, cords[1].1),
                cords[0].0,
                cords[0].1,
            ));
            beacons.push(cords[1]);

            min_x = min_x.min(cords[0].0 - sensors[sensors.len() - 1].dist);
            max_x = max_x.max(cords[0].0 + sensors[sensors.len() - 1].dist);
        } else {
            return Err(InputError::InvalidInput);
        }
    }

    Ok(Input::D15((sensors, beacons, (min_x, max_x))))
}
