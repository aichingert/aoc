use crate::day::{
    d15::{CellType, Entity},
    Input, InputResult, Output, Point, Wrapper,
};
use std::collections::HashSet;

const VEC: [(i16, i16); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

fn print(grid: &Vec<Vec<CellType>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            print!(
                "{}",
                match grid[y][x] {
                    CellType::Empty => '.',
                    CellType::Wall => '#',
                    CellType::Elve => 'E',
                    CellType::Goblin => 'G',
                }
            );
        }
        println!()
    }
}

fn part_one(entities: &Vec<Entity>, grid: &mut Vec<Vec<CellType>>) -> u32 {
    let mut entities = entities.clone();
    let mut round: u32 = 0;

    loop {
        let mut filtered_dead_entities = Vec::<Entity>::new();

        for i in 0..entities.len() {
            if entities[i].hp > 0 {
                filtered_dead_entities.push(entities[i]);
            } else {
                let (y, x) = entities[i].pos;
                grid[y][x] = CellType::Empty;
            }
        }

        entities = filtered_dead_entities;
        entities.sort_by_key(|e| (e.pos.0, e.pos.1));
        if round == 69 {
            println!("{round}");
            print(grid);

            for j in 0..entities.len() {
                println!("{:?}", entities[j]);
            }
            println!();
        }

        if round == 70 {
            println!("{round}");
            print(grid);

            for j in 0..entities.len() {
                println!("{:?}", entities[j]);
            }
            println!();
        }
        for i in 0..entities.len() {
            if entities[i].hp > 0 && !is_enemy_alive(&entities, entities[i].is_elve) {
                print(grid);

                for j in 0..entities.len() {
                    println!("{:?}", entities[j]);
                }
                return round * entities.iter().map(|e| e.hp.max(0) as u32).sum::<u32>();
            }

            for j in 0..entities.len() {
                if entities[j].hp < -1 {
                    println!("WARNING");
                }
                //println!("{:?}", entities[j]);
            }

            if entities[i].hp <= 0 || is_next_to_enemy(grid, i, &mut entities) {
                continue;
            }

            let dp = map_distance(grid, entities[i].pos);

            let mut best: u32 = u32::MAX;
            let mut target: Option<Point<usize>> = None;

            for enemie in entities
                .iter()
                .filter(|e| e.is_elve != entities[i].is_elve)
                .collect::<Vec<&Entity>>()
            {
                for (y, x) in get_open_fields(grid, enemie.pos) {
                    if dp[y][x] > 10000 {
                        continue;
                    }

                    if dp[y][x] < best {
                        best = dp[y][x];
                        target = Some((y, x));
                    } else if dp[y][x] == best && target.is_some_and(|(dy, dx)| dy >= y && dx >= x)
                    {
                        best = dp[y][x];
                        target = Some((y, x));
                    }
                }
            }

            if let Some((y, x)) = target {
                let dp = map_distance(grid, (y, x));
                best = u32::MAX;
                target = None;

                for (y, x) in get_open_fields(grid, entities[i].pos) {
                    if dp[y][x] > 10000 {
                        continue;
                    }
                    if dp[y][x] < best {
                        best = dp[y][x];
                        target = Some((y, x));
                    } else if dp[y][x] == best && target.is_some_and(|(dy, dx)| dy >= y && dx >= x)
                    {
                        best = dp[y][x];
                        target = Some((y, x));
                    }
                }

                if let Some((y, x)) = target {
                    let (py, px) = entities[i].pos;
                    grid[py][px] = CellType::Empty;
                    grid[y][x] = if entities[i].is_elve {
                        CellType::Elve
                    } else {
                        CellType::Goblin
                    };
                    entities[i].pos = (y, x);

                    is_next_to_enemy(grid, i, &mut entities);
                }
            }
        }
        round += 1;
    }
}

fn is_enemy_alive(entities: &Vec<Entity>, is_elve: bool) -> bool {
    entities
        .iter()
        .find(|e| e.is_elve != is_elve && e.hp > 0)
        .is_some()
}

fn is_next_to_enemy(grid: &Vec<Vec<CellType>>, entity: usize, entities: &mut Vec<Entity>) -> bool {
    let enemy = if entities[entity].is_elve {
        CellType::Goblin
    } else {
        CellType::Elve
    };
    let mut min_hp = i16::MAX;
    let mut to_hit = None::<usize>;

    for (dy, dx) in &VEC {
        let (y, x) = (
            (entities[entity].pos.0 as i16 + dy) as usize,
            (entities[entity].pos.1 as i16 + dx) as usize,
        );

        if grid[y][x] == enemy {
            for i in 0..entities.len() {
                if entities[i].hp > 0 && entities[i].pos == (y, x) && entities[i].hp < min_hp {
                    min_hp = entities[i].hp;
                    to_hit = Some(i);
                }
            }
        }
    }

    if let Some(idx) = to_hit {
        entities[idx].hp -= entities[entity].dmg;
        true
    } else {
        false
    }
}

fn get_open_fields(grid: &Vec<Vec<CellType>>, cur: Point<usize>) -> Vec<Point<usize>> {
    let mut open = Vec::new();

    for (dy, dx) in &VEC {
        let (y, x) = ((cur.0 as i16 + dy) as usize, (cur.1 as i16 + dx) as usize);
        match grid[y][x] {
            CellType::Empty => open.push((y, x)),
            _ => {}
        }
    }

    open
}

fn map_distance(grid: &Vec<Vec<CellType>>, start: Point<usize>) -> Vec<Vec<u32>> {
    let mut dp = vec![vec![u32::MAX - 2; grid[0].len()]; grid.len()];
    let mut s: HashSet<Point<usize>> = HashSet::from([start]);
    let mut q: Vec<Point<usize>> = vec![start];

    dp[start.0][start.1] = 0;

    while !q.is_empty() {
        let u = {
            let mut found = None;
            let mut best = u32::MAX;

            for i in 0..q.len() {
                let (y, x) = (q[i].0, q[i].1);

                if dp[y][x] + 1 < best {
                    best = dp[y][x];
                    found = Some(i);
                }
            }

            found.unwrap()
        };

        let e = q.remove(u);
        s.insert(e);

        for n in get_open_fields(grid, e) {
            if !s.contains(&n) && !q.contains(&n) {
                q.push(n);
            }
            let cur = dp[n.0][n.1];

            if cur > dp[e.0][e.1] + 1 {
                dp[n.0][n.1] = dp[e.0][e.1] + 1;
            }
        }
    }

    dp
}

pub fn run(input: Input) -> (Output, Output) {
    let (mut entities, mut grid): (Vec<Entity>, Vec<Vec<CellType>>) = input.unwrap();

    println!("{:?}", part_one(&mut entities, &mut grid));

    (Output::None, Output::None)
}

pub fn parse() -> InputResult<Input> {
    let mut grid: Vec<Vec<CellType>> = Vec::new();
    let mut entities: Vec<Entity> = Vec::new();

    for (y, line) in std::fs::read_to_string("../input/15")?.lines().enumerate() {
        grid.push(Vec::new());
        for (x, ch) in line.chars().enumerate() {
            grid[y].push(match ch {
                '#' => CellType::Wall,
                '.' => CellType::Empty,
                'E' | 'G' => {
                    entities.push(Entity {
                        hp: 200,
                        dmg: 3,
                        pos: (y, x),
                        is_elve: ch == 'E',
                    });

                    if ch == 'E' {
                        CellType::Elve
                    } else {
                        CellType::Goblin
                    }
                }
                _ => return Err(crate::day::InputError::InvalidInput),
            });
        }
    }

    Ok(Input::D15((entities, grid)))
}
