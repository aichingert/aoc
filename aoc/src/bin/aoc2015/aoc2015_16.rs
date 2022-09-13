use aoc::read_to_slice;

pub struct Aoc2015_16 {
    aunts: Vec<Aunt>
}

#[derive(Debug)]
struct Aunt {
    id: i32,
    children: i32,
    cats: i32,
    samoyeds: i32,
    pomeranians: i32,
    akitas: i32,
    vizslas: i32,
    goldfish: i32,
    trees: i32,
    cars: i32,
    perfumes: i32
}

impl Aunt {
    fn new() -> Self {
        Self {
            id: -1,
            children: -1,
            cats: -1,
            samoyeds: -1,
            pomeranians: -1,
            akitas: -1,
            vizslas: -1,
            goldfish: -1,
            trees: -1,
            cars: -1,
            perfumes: -1
        }
    }
}

impl Aoc2015_16 {
    pub fn new() -> Self {
        Self { aunts: vec![] }
    }
}
        
impl crate::Solution for Aoc2015_16 {
    fn name(&self) -> (usize, usize) {
        (2015, 16)
    }
        
    fn parse(&mut self) {
        let p = read_to_slice("input/2015/16.txt", " ");

        for i in 0..p.len() {
            let mut aunt = Aunt::new();
            aunt.id = p[i][1][0..p[i][1].len() - 1].parse().unwrap();

            for j in 2..p[i].len() {
                if j + 1 < p[i].len() {
                    let value: Result<i32, std::num::ParseIntError> = if j + 1 == p[i].len() - 1 {
                        p[i][j + 1].parse()
                    } else {
                        p[i][j + 1][0..p[i][j + 1].len() - 1].parse()
                    };

                    if let Some(n) = value.ok() {
                        match &p[i][j][0..p[i][j].len()-1] {
                            "children" => aunt.children = n,
                            "cats" => aunt.cats = n,
                            "samoyeds" => aunt.samoyeds = n,
                            "pomeranians" => aunt.pomeranians = n,
                            "akitas" => aunt.akitas = n,
                            "vizslas" => aunt.vizslas = n,
                            "goldfish" => aunt.goldfish = n,
                            "trees" => aunt.trees = n,
                            "cars" => aunt.cars = n,
                            "perfumes" => aunt.perfumes = n,
                            _ => {}
                        }
                    }
                }
            }

            self.aunts.push(aunt);
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        let searching: Aunt = Aunt {
            id: 0,
            children: 3,
            cats: 7,
            samoyeds: 2,
            pomeranians: 3,
            akitas: 0,
            vizslas: 0,
            goldfish: 5,
            trees: 3,
            cars: 2,
            perfumes: 1
        };
        let mut id: (i32, i32) = (0, 0);

        for i in 0..self.aunts.len() {
            let mut similaritys: i32 = 0;
            if searching.children == self.aunts[i].children {similaritys += 1}
            if searching.cats == self.aunts[i].cats {similaritys += 1}
            if searching.samoyeds == self.aunts[i].samoyeds {similaritys += 1}
            if searching.pomeranians == self.aunts[i].pomeranians {similaritys += 1}
            if searching.akitas == self.aunts[i].akitas {similaritys += 1}
            if searching.vizslas == self.aunts[i].vizslas {similaritys += 1}
            if searching.goldfish == self.aunts[i].goldfish {similaritys += 1}
            if searching.trees == self.aunts[i].trees {similaritys += 1}
            if searching.cars == self.aunts[i].cars {similaritys += 1}
            if searching.perfumes == self.aunts[i].perfumes {similaritys += 1}

            if similaritys > id.1 {
                id = (self.aunts[i].id, similaritys);
            }
        }
        crate::output(id.0)
    }
        
    fn part2(&mut self) -> Vec<String> {
        crate::output("")
    }
}