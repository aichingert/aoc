use std::collections::HashSet;

fn main() {
    let inp = std::fs::read_to_string("../input/23").unwrap().trim().to_string();
    let mut elves = HashSet::new();

    for (i, e) in inp.lines().enumerate() {
        for (j, c) in e.chars().enumerate() {
            if c == '#' {
                elves.insert((i as i32, j as i32));
            }
        }
    }

    let mut directions = vec![D::N, D::S, D::W, D::E];

    for n in 0..10000 {
        let mut next = HashSet::new();
        let (cont, proposes) = game_loop(&elves, &mut directions);

        if !cont {
            println!("round: {}", n+1);
            break;
        }

        for i in 0..proposes.len() {
            let mut blocked = false;
            for j in 0..proposes.len() {
                if blocked { break; }
                if i == j { continue; }

                blocked = match (proposes[i].0, proposes[j].0) {
                    (D::N, D::S) => {
                        let ((iy, ix),(jy, jx)) = (proposes[i].1, proposes[j].1);
                        iy - 1 == jy + 1 && ix == jx
                    }
                    (D::S, D::N) => {
                        let ((iy, ix), (jy, jx)) = (proposes[i].1, proposes[j].1);
                        iy + 1 == jy - 1 && ix == jx
                    },
                    (D::W, D::E) => {
                        let ((iy, ix),(jy, jx)) = (proposes[i].1, proposes[j].1);
                        iy == jy && ix - 1 == jx + 1
                    },
                    (D::E, D::W) => {
                        let ((iy, ix),(jy, jx)) = (proposes[i].1, proposes[j].1);
                        iy == jy && ix + 1 == jx - 1
                    }
                    _ => false,
                }
            }

            if blocked {
                next.insert(proposes[i].1);
            } else {
                next.insert(proposes[i].0.apply(proposes[i].1));
            }
        }

        elves = next;
    }

    let (mut miy, mut may) = (i32::MAX, i32::MIN);
    let (mut mix, mut max) = (i32::MAX, i32::MIN);

    for (y, x) in elves.iter() {
        if *y < miy { miy = *y; }
        if *y > may { may = *y; }
        if *x < mix { mix = *x; }
        if *x > max { max = *x; }
    }

    println!("{:?}", (may - miy + 1) * (max - mix + 1) - elves.len() as i32);
}

#[derive(Debug, Clone, Copy)]
enum D {
    N,
    S,
    W,
    E,
    None,
}

impl D {
    fn apply(&self, vec: (i32, i32)) -> (i32, i32) {
        match self {
            D::N => (vec.0 - 1, vec.1),
            D::S => (vec.0 + 1, vec.1),
            D::W => (vec.0, vec.1 - 1),
            D::E => (vec.0, vec.1 + 1),
            D::None => vec,
        }
    }
}

fn game_loop(elves: &HashSet<(i32, i32)>, directions: &mut Vec<D>) -> (bool, Vec<(D, (i32, i32))>) {
    let mut proposes = Vec::new();
    let mut cont = false;

    'outer: for (dy, dx) in elves.iter() {
        let (dy, dx) = (*dy, *dx);

        let has_neighbour = {
            let mut value = false;
            for d in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)].iter() {
                if elves.contains(&(dy + d.0, dx + d.1)) {
                    value = true;
                    break;
                }
            }

            value
        };

        if has_neighbour {
            cont = true;
            for i in 0..directions.len() {
                match directions[i] {
                    D::N => if !elves.contains(&(dy - 1, dx)) 
                            && !elves.contains(&(dy - 1, dx + 1)) 
                            && !elves.contains(&(dy - 1, dx - 1)) {
                            proposes.push((D::N, (dy, dx)));
                            continue 'outer;
                    },
                    D::S => if !elves.contains(&(dy + 1, dx)) 
                            && !elves.contains(&(dy + 1, dx + 1)) 
                            && !elves.contains(&(dy + 1, dx - 1)) {
                            proposes.push((D::S, (dy, dx)));
                            continue 'outer;
                    },
                    D::W => if !elves.contains(&(dy, dx - 1)) 
                            && !elves.contains(&(dy - 1, dx - 1)) 
                            && !elves.contains(&(dy + 1, dx - 1)) {
                            proposes.push((D::W, (dy, dx)));
                            continue 'outer;
                    },
                    D::E => if !elves.contains(&(dy, dx + 1)) 
                            && !elves.contains(&(dy + 1, dx + 1)) 
                            && !elves.contains(&(dy - 1, dx + 1)) {
                            proposes.push((D::E, (dy, dx)));
                            continue 'outer;
                    },
                    _ => {},
                }
            }
        }

        proposes.push((D::None, (dy, dx)));
    }

    let first = directions.remove(0);
    directions.push(first);

    (cont, proposes)
}
