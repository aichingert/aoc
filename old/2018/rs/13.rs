// Advent of Code 2018, day 13
// (c) aichingert

use std::collections::HashSet;
const TURNS: [i32;3] = [-1,0,1];

struct Car {
    pos: (usize,usize),
    dir: (i32,i32),
    turn: usize,
}

impl Car {
    fn new(pos: (usize,usize), dir: (i32,i32)) -> Self {
        Self { pos, dir, turn: 0 }
    }

    fn update(&mut self) {
        self.pos = ((self.pos.0 as i32 + self.dir.0) as usize,(self.pos.1 as i32 + self.dir.1) as usize)
    }

    fn turn(&mut self, turn: char) {
        match turn {
            '/' => {
                match self.dir {
                    (1, 0) => self.dir = (0,-1), // Oben -> Links
                    (-1,0) => self.dir = (0,1), // Unten -> Rechts
                    (0,-1) => self.dir = (1,0), // Links -> 
                    (0, 1) => self.dir = (-1,0),
                    _ => (),
                }
            },
            '\\' => {
                match self.dir {
                    (1, 0) => self.dir = (0,1),
                    (-1,0) => self.dir = (0,-1),
                    (0,-1) => self.dir = (-1,0),
                    (0, 1) => self.dir = (1,0),
                    _ => (),
                }
            },
            '+' => {
                match TURNS[self.turn] {
                    -1 => {
                        self.dir = match self.dir {
                            (1,0) => (0,1),
                            (-1,0) => (0,-1),
                            (0,1) => (-1,0),
                            (0,-1) => (1,0),
                            _ => panic!("invalid dir")
                        }
                    },
                    1 => {
                        self.dir = match self.dir {
                            (1,0) => (0,-1),
                            (-1,0) => (0,1),
                            (0,1) => (1,0),
                            (0,-1) => (-1,0),
                            _ => panic!("invalid dir")
                        }
                    }
                    _ => (),
                };
                self.turn = (self.turn + 1) % TURNS.len();
            },
            _ => (),
        };

        self.update();
    }
}

fn part1(map: &Vec<Vec<char>>, cars: &mut Vec<Car>) -> (usize,usize) {
    let mut res: Vec<(usize,usize)> = Vec::new();
    let mut r = 0;
    loop {
        r += 1;
        for i in 0..cars.len() {
            if i % 2 == 0 {
                //pr(map, cars[i].pos);
            }

            let cur = map[cars[i].pos.0][cars[i].pos.1];
            cars[i].turn(cur);

            for j in i+1..cars.len() {
                if cars[i].pos == cars[j].pos {
                    res.push(cars[i].pos);
                    println!("{r}");
                }
            }
        }

        if res.len() > 2 { break; }
    }

    println!("{:?}", res);
    return res[0];
}

fn part2(map: &Vec<Vec<char>>, cars: &mut Vec<Car>) -> (usize,usize) {
    while cars.len() > 1 {
        let mut i: usize = 0;

        while i < cars.len() {
            let mut to_rem = HashSet::new();
            for _ in 0..1000000 {
                print!("");
            }
            if i == 0 {
                pr(map, cars);
            }


            let cur = map[cars[i].pos.0][cars[i].pos.1];
            cars[i].turn(cur);

            for j in i+1..cars.len() {
                if cars[i].pos == cars[j].pos {
                    to_rem.insert(i);
                    to_rem.insert(j);
                }
            }

            if to_rem.len() > 0 {
                let mut offset = 0;
                let mut vec = to_rem.iter().map(|n| *n).collect::<Vec<usize>>();
                vec.sort();
                for i in 0..vec.len() {
                    cars.remove(vec[i - offset]);
                    offset += 1;
                }
            }
        }
    }

    cars[0].pos
}

fn pr(map: &Vec<Vec<char>>, pos: &Vec<Car>) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let mut con = false;
            for x in 0..pos.len() {
                if pos[x].pos == (i,j) {
                    con = true;
                }
            }

            if con {
                print!("#")
            } else {
                print!("{}",map[i][j]);
            }
        }
        println!("");
    }
    println!("");
}

fn parse() -> (Vec<Vec<char>>, Vec<Car>) {
    let mut cars = Vec::<Car>::new();
    let inp = std::fs::read_to_string("../input/13").unwrap();
    let inp = inp
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for i in 0..inp.len() {
        for j in 0..inp[i].len() {
            match inp[i][j] {
                '^' => cars.push(Car::new((i,j),(-1,0))),
                'v' => cars.push(Car::new((i,j),(1,0))),
                '>' => cars.push(Car::new((i,j),(0,1))),
                '<' => cars.push(Car::new((i,j),(0,-1))),
                _ => (),
            };
        }
    }

    (inp,cars)
}

fn main() {
    let (map, mut cars) = parse();

    //println!("Part 1: {:?}", part1(&map, &mut cars));
    println!("Part 2: {:?}", part2(&map, &mut cars));
}
