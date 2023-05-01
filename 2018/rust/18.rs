// Advent of Code 2018, day 18
// (c) aichingert

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Type {
    Tree,
    Ground,
    Lumberyard,
}

impl Type {
    fn count(check: Self, area: &Vec<Vec<Type>>, loc: (usize,usize)) -> u8 {
        let mut count: u8 = 0;

        for i in -1..2 {
            for j in -1..2 {
                if i == 0 && j == 0 {continue;}

                let y = loc.0 as i32 + i;
                let x = loc.1 as i32 + j;

                
                if y > -1 && x > -1 && (y as usize) < area.len() && (x as usize) < area[0].len() 
                && area[y as usize][x as usize] == check {
                    count += 1;
                }
            }
        }
        
        count 
    }

    fn next(&self, area: &Vec<Vec<Type>>, loc: (usize,usize)) -> Self {
        match self {
            Type::Ground => {
                let count = Type::count(Type::Tree, area, loc);

                if count > 2 {
                    Self::Tree
                } else {
                    *self
                }
            },
            Type::Tree => {
                let count = Type::count(Type::Lumberyard, area, loc);

                if count > 2 {
                    Self::Lumberyard
                } else {
                    *self
                }
            },
            Type::Lumberyard => {
                let trees = Type::count(Type::Tree, area, loc);
                let lumberyards = Type::count(Type::Lumberyard, area, loc);

                if trees > 0 && lumberyards > 0 {
                    *self
                } else {
                    Self::Ground
                }
            }
        }
    }
}

fn solve(area: &Vec<Vec<Type>>, minutes: i64) -> usize {
    let mut cur = area.clone();

    for _ in 0..minutes {
        let mut next = vec![vec![Type::Ground;area.len()];area.len()];

        for i in 0..area.len() {
            for j in 0..area[i].len() {
                next[i][j] = cur[i][j].next(&cur, (i,j));
            }
        }

        cur = next;
    }

    let mut trees = 0;
    let mut lumberyards = 0;

    for i in 0..cur.len() {
        for j in 0..cur[i].len() {
            match cur[i][j] {
                Type::Tree => trees += 1,
                Type::Lumberyard => lumberyards += 1,
                Type::Ground => (),
            };
        }
    }

    trees * lumberyards
}

fn main() {
    let inp = std::fs::read_to_string("../input/18").unwrap()
        .lines()
        .map(|l| l.chars()
             .map(|c| match c {
                 '|' => Type::Tree,
                 '.' => Type::Ground,
                 '#' => Type::Lumberyard,
                 _ => panic!("invalid input"),
             }).collect::<Vec<Type>>()
        ).collect::<Vec<Vec<Type>>>();

    println!("Part 1: {}", solve(&inp, 10));
}
