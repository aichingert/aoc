use std::collections::HashMap;

fn main() {
    let input: String = std::fs::read_to_string("input.txt").expect("err");

    solve_part_one(&input);
}

#[derive(Debug, Clone)]
struct Particle {
    id: i64,
    p: (i64, i64, i64),
    v: (i64, i64, i64),
    a: (i64, i64, i64),
}

fn solve_part_one(input: &String) {
    let mut particles: Vec<Particle> = Vec::new();
    let mut ids: HashMap<i64, i64> = HashMap::new(); 
    let mut id_count: i64 = 0;
    
    for line in input.lines() {
        let values: Vec<_> = line.split(", ").collect();

        let mut first_split: Vec<_> = values[0].split('<').collect();
        let mut second_split: Vec<_> = first_split[1].split('>').collect();
        let mut numbers: Vec<_> = second_split[0].split(',').collect();

        let p: (i64, i64, i64) = (numbers[0].parse::<i64>().unwrap(), numbers[1].parse::<i64>().unwrap(), numbers[2].parse::<i64>().unwrap());
        
        first_split = values[1].split('<').collect();
        second_split = first_split[1].split('>').collect();
        numbers = second_split[0].split(',').collect();

        let v: (i64, i64, i64) = (numbers[0].parse::<i64>().unwrap(), numbers[1].parse::<i64>().unwrap(), numbers[2].parse::<i64>().unwrap());
        
        first_split = values[2].split('<').collect();
        second_split = first_split[1].split('>').collect();
        numbers = second_split[0].split(',').collect();

        let a: (i64, i64, i64) = (numbers[0].parse::<i64>().unwrap(), numbers[1].parse::<i64>().unwrap(), numbers[2].parse::<i64>().unwrap());

        let particle: Particle = Particle { id: id_count, p: p, v: v, a: a };
        particles.push(particle);
        id_count += 1;
    }

    for part in &mut particles {
        for _ in 0..0 {
            update(part);
        }

        ids.insert(part.id, part.a.0.abs() + part.a.1.abs() + part.a.2.abs());
    }

    let mut closest: i64 = 1000000;
    let mut id: i64 = 0;

    for key in ids.keys() {
        if ids[key] < closest {
            closest = ids[key];
            id = *key;
        } else if ids[key] == closest
        && particles[*key as usize].p.0.abs() + particles[*key as usize].p.1.abs() + particles[*key as usize].p.2.abs() < particles[id as usize].p.0.abs() + particles[id as usize].p.1.abs() + particles[id as usize].p.2.abs() {
            closest = ids[key];
            id = *key;
        }
    }

    println!("Solution part one: {}", id);

    solve_part_two(&mut particles);
}

fn solve_part_two(particles: &mut Vec<Particle>) {
    for _ in 0..1000 { // if wrong, increase counter for the loop
        let mut new_particles: Vec<Particle> = particles.clone();
        let mut to_remove: Vec<usize> = Vec::new();
        for part in &mut new_particles { 
            update(part);
        }

        *particles = new_particles;

        for i in 0..particles.len() {
            for j in 0..particles.len() {
                if i == j {
                    continue;
                }

                if particles[i].p == particles[j].p && !to_remove.contains(&j) {
                    to_remove.push(j);
                }
            }
        }

        let mut fix: usize = 0;
        to_remove.sort();

        for rem in &to_remove {
            particles.remove(rem - fix);
            fix += 1;
        }
    }

    println!("Solution part two: {}", particles.len());
}

fn update(particle: &mut Particle) {
    particle.v.0 += particle.a.0;
    particle.v.1 += particle.a.1;
    particle.v.2 += particle.a.2;

    particle.p.0 += particle.v.0;
    particle.p.1 += particle.v.1;
    particle.p.2 += particle.v.2;
}