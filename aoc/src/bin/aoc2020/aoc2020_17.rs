use std::collections::HashMap;

pub struct Aoc2020_17 {
    cubes_3d: HashMap<(i32, i32, i32), bool>,
    cubes_4d: HashMap<(i32,i32,i32,i32), bool>,
    size: i32
}
        
impl Aoc2020_17 {
    pub fn new() -> Self {
        Self { 
            cubes_3d: HashMap::new(),
            cubes_4d: HashMap::new(),
            size: 0
        }
    }
}
        
impl crate::Solution for Aoc2020_17 {
    fn name(&self) -> (usize, usize) {
        (2020, 17)
    }
        
    fn parse(&mut self) {
        let input = std::fs::read_to_string("input/2020/17.txt").expect("unable to open file!");
        let input: Vec<&str> = input.lines().collect();
        self.size = input.len() as i32;

        for x in 0..input.len() {
            for (y,c) in input[x].chars().enumerate() {
                if c == '#' {
                    self.cubes_3d.insert((x as i32,y as i32,0), true);
                    self.cubes_4d.insert((x as i32, y as i32, 0, 0), true);
                }
            }
        }
    }
        
    fn part1(&mut self) -> Vec<String> {
        let mut size: i32 = self.size;

        for _cycles in 0..6 {
            let mut new_cubes: HashMap<(i32, i32, i32), bool> = HashMap::new();
            size += 2;

            for x in -size..size {
                for y in -size..size {
                    for z in -size..size {
                        let mut neighbours: u8 = 0;

                        for xx in -1..2 {
                            for yy in -1..2 {
                                for zz in -1..2 {
                                    if yy == 0 && xx == 0 && zz == 0 {
                                        continue;
                                    }

                                    if self.cubes_3d.contains_key(&(x+xx, y+yy, z+zz)) {
                                        neighbours += 1;
                                    }
                                }
                            }
                        }

                        if self.cubes_3d.contains_key(&(x, y, z)) && neighbours >= 2 && neighbours <= 3 {
                            new_cubes.insert((x,y,z), true);
                        } else if neighbours == 3 {
                            new_cubes.insert((x,y,z), true);
                        }
                    }
                }
            }

            self.cubes_3d = new_cubes;
        }

        crate::output(self.cubes_3d.len())
    }
        
    fn part2(&mut self) -> Vec<String> {
        let mut size: i32 = self.size;

        for _cycles in 0..6 {
            let mut new_cubes: HashMap<(i32, i32, i32, i32), bool> = HashMap::new();
            size += 2;

            for x in -size..size {
                for y in -size..size {
                    for z in -size..size {
                        for w in -size..size {
                            let mut neighbours: u8 = 0;

                            for xx in -1..2 {
                                for yy in -1..2 {
                                    for zz in -1..2 {
                                        for ww in -1..2 {
                                            if yy == 0 && xx == 0 && zz == 0 && ww == 0 {
                                                continue;
                                            }
        
                                            if self.cubes_4d.contains_key(&(x+xx, y+yy, z+zz, w+ww)) {
                                                neighbours += 1;
                                            }
                                        }
                                    }
                                }
                            }
    
                            if self.cubes_4d.contains_key(&(x, y, z, w)) && neighbours >= 2 && neighbours <= 3 {
                                new_cubes.insert((x,y,z,w), true);
                            } else if neighbours == 3 {
                                new_cubes.insert((x,y,z,w), true);
                            }
                        }
                    }
                }
            }

            self.cubes_4d = new_cubes;
        }

        crate::output(self.cubes_4d.len())
    }
}