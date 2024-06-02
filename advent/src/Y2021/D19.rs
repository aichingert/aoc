pub fn solve() {}

/*
fn roll(cord: &[i32]) -> Vec<i32> {
    vec![cord[0],cord[2], -cord[1]]
}

fn turn(cord: &[i32]) -> Vec<i32> {
    vec![-cord[1], cord[0], cord[2]]
}

// https://stackoverflow.com/questions/16452383/how-to-get-all-24-rotations-of-a-3-dimensional-array
fn orientations(cord: &[i32]) -> Vec<Vec<i32>> {
    let mut ors = Vec::new();
    let mut cord = cord.to_vec();

    for _ in 0..2 {
        for _ in 0..3 {
            cord = roll(&cord);
            ors.push(cord.clone());

            for _ in 0..3 {
                cord = turn(&cord);
                ors.push(cord.clone());
            }
        }
        cord = roll(&turn(&roll(&cord)));
    }

    ors
}

pub fn solve() {
    let inp = std::fs::read_to_string("input/2021/19").unwrap().trim().to_string();

    let sensors = inp.split("\n\n").map(|sensor| {
        let mut lines = sensor.lines();
        let sensor_id = lines.next().unwrap().split(' ').nth(2).unwrap().parse::<u32>().unwrap();
        let beacons   = lines.map(|line| line.split(',').map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

        (sensor_id, beacons)
    })
    .collect::<Vec<_>>();


    // Sensor 1
    //
    // Beacon1...
    // Beacon2...

    // Sensor 2
    //
    // Beacon2...
    // Beacon3...

    let mut diffs = Vec::new();

    for b_x in 0..sensors[0].1.len() {

        // BeaconX...

        // SensorX...
        for s_x in 1..sensors.len()  {
            // BeaconXs

            for b_xs in 0..sensors[s_x].1.len() {
                let curr = orientations(&sensors[0].1[b_x]);
                let this = orientations(&sensors[s_x].1[b_xs]);

                for i in 0..curr.len() {
                    for j in 0..this.len() {
                        let x = curr[i][0] - this[i][0];
                        let y = curr[i][1] - this[i][1];
                        let z = curr[i][2] - this[i][2];

                        diffs.push((sensors[0].1[b_x].clone(), (x,y,z)));
                    }
                }
            }
        }
    }

    diffs.sort_unstable();

    for d in diffs {
        println!("{d:?}");
    }

}
*/
