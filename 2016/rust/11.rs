// Advent of Code 2016, day 2
// (c) aichingert

use std::collections::HashSet;

#[derive(Clone)]
struct Tower {
    floors: Vec<Floor>,
    elevator: usize,
}

#[derive(Clone)]
struct Floor {
    microchips: HashSet<Item>,
    generators: HashSet<Item>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Item {
    Microchip(String),
    Generator(String),
}

impl Tower {
    fn from_str() -> Self {
        let mut tower: Tower = Tower { floors: Vec::new(), elevator: 0 };

        for line in std::fs::read_to_string("../input/11").unwrap().lines() {
            let mut floor = Floor::new();
            let mut iter = line.split(' ').skip(5);

            while let Some(word) = iter.next() {
                if word == "a" || word == "and" || word == "relevant." || word.starts_with("micro") || word.starts_with("gener") {
                    continue;
                }

                if let Some((lhs,rhs)) = word.split_once('-') {
                    floor.microchips.insert(Item::Microchip(lhs.to_string()));
                } else {
                    floor.generators.insert(Item::Generator(word.to_string()));
                }
            }

            tower.floors.push(floor);
        }

        tower
    }

    fn next(&self) -> Vec<Tower> {
        let mut towers = Vec::<Tower>::new();
        let items = self.floors[self.elevator]
            .microchips.iter()
            .chain(self.floors[self.elevator].generators.iter())
            .collect::<Vec<&Item>>();

        for item in items.iter() {
            if self.elevator > 0 {
                let mut tower = self.clone();
                tower.elevator -= 1;
                tower.floors[self.elevator].remove(item);
                tower.floors[tower.elevator].insert(item);

                if tower.floors[self.elevator].is_valid() && tower.floors[tower.elevator].is_valid() {
                    towers.push(tower);
                }
            }

            if self.elevator < self.floors.len()-1 {
                let mut tower = self.clone();
                tower.elevator += 1;
                tower.floors[self.elevator].remove(item);
                tower.floors[tower.elevator].insert(item);
                if tower.floors[self.elevator].is_valid() && tower.floors[tower.elevator].is_valid() {
                    towers.push(tower);
                }
            }
        }

        for i in 0..items.len()-1 {
            for j in i+1..items.len() {
                if self.elevator > 0 {
                    let mut tower = self.clone();
                    tower.elevator -= 1;
                    tower.floors[self.elevator].remove(items[i]);
                    tower.floors[self.elevator].remove(items[j]);
                    tower.floors[tower.elevator].insert(items[i]);
                    tower.floors[tower.elevator].insert(items[j]);

                    if tower.floors[self.elevator].is_valid() && tower.floors[tower.elevator].is_valid() {
                        towers.push(tower);
                    }
                }
                
                if self.elevator < self.floors.len()-1 {
                    let mut tower = self.clone();
                    tower.elevator += 1;
                    tower.floors[self.elevator].remove(items[i]);
                    tower.floors[self.elevator].remove(items[j]);
                    tower.floors[tower.elevator].insert(items[i]);
                    tower.floors[tower.elevator].insert(items[j]);

                    if tower.floors[self.elevator].is_valid() && tower.floors[tower.elevator].is_valid() {
                        towers.push(tower);
                    }
                }
            }
        }

        towers
    }
}

impl Floor {
    fn new() -> Self {
        Self { microchips: HashSet::new(), generators: HashSet::new() }
    }

    fn remove(&mut self, item: &Item) {
        match item {
            Item::Microchip(_) => self.microchips.remove(item),
            Item::Generator(_) => self.generators.remove(item),
        };
    }

    fn insert(&mut self, item: &Item) {
        match item {
            Item::Microchip(_) => self.microchips.insert(item.clone()),
            Item::Generator(_) => self.generators.insert(item.clone()),
        };
    }

    fn is_valid(&self) -> bool {
        if self.microchips.is_empty() || self.generators.is_empty() {
            return true;
        }

        for micro in self.microchips.iter() {
            if !self.generators.contains(&micro.reverse()) {
                return false;
            }
        }

        true
    }
}

impl Item {
    fn reverse(&self) -> Self {
        match self {
            Item::Microchip(s) => Item::Generator(s.clone()),
            Item::Generator(s) => Item::Microchip(s.clone()),
        }
    }
}

fn part1() {}
fn part2() {}

fn main() {
    let tower = Tower::from_str();
}
