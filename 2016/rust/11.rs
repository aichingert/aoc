// Advent of Code 2016, day 2
// (c) aichingert

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone,Hash, Eq, PartialEq)]
struct Tower {
    floors: Vec<Floor>,
    elevator: usize,
}

#[derive(Clone,Eq,PartialEq)]
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
                if word == "a" || word == "and" || word == "relevant." 
                    || word.starts_with("micro") || word.starts_with("gener") {
                    continue;
                }

                if let Some((lhs,_)) = word.split_once('-') {
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

    fn finished(&self) -> bool {
        for i in 0..self.floors.len()-1 {
            if !(self.floors[i].microchips.is_empty() && self.floors[i].generators.is_empty()) {
                return false;
            }
        }

        true
    }
}

impl Hash for Floor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.microchips.len().hash(state);
        self.generators.len().hash(state);
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

fn part1(starting: &Tower) -> u32 {
    let mut batch = vec![starting.clone()];
    let mut next_batch = Vec::new();
    let mut dist = 0;
    let mut seen = HashSet::new();

    loop {
        for st in batch {
            for n in st.next() {
                if seen.contains(&n) {
                    continue;
                }

                if n.finished() {
                    return dist + 1;
                }

                next_batch.push(n.clone());
                seen.insert(n);
            }
        }

        batch = next_batch.clone();
        next_batch.clear();
        dist += 1;
    }
}

fn main() {
    let tower = Tower::from_str();
    println!("Part 1: {}", part1(&tower));
}
