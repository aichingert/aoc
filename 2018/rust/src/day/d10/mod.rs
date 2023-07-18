pub mod the_stars_align;

use crate::day::Point;

pub struct Star {
    pos: Point<i32>,
    vel: Point<i32>,
}

impl Star {
    pub fn new(pos: Point<i32>, vel: Point<i32>) -> Self {
        Self { pos, vel }
    }

    pub fn update(&mut self) {
        self.pos.0 += self.vel.0;
        self.pos.1 += self.vel.1;
    }

    pub fn furthest_points(stars: &Vec<Self>) -> (i32, i32, i32, i32) {
        let (mut le_x, mut ri_x, mut bo_y, mut to_y) = (i32::MAX, -i32::MAX, i32::MAX, -i32::MAX);

        for star in stars.iter() {
            if star.pos.0 < le_x {
                le_x = star.pos.0;
            }
            if star.pos.0 > ri_x {
                ri_x = star.pos.0;
            }
            if star.pos.1 < bo_y {
                bo_y = star.pos.1;
            }
            if star.pos.1 > to_y {
                to_y = star.pos.1;
            }
        }

        (le_x, ri_x, bo_y, to_y)
    }

    pub fn print(stars: &Vec<Self>, le_x: i32, ri_x: i32, bo_y: i32, to_y: i32) {
        for i in bo_y - 1..to_y + 1 {
            for j in le_x - 1..ri_x + 1 {
                if stars.iter().find(|x| x.pos == (j, i)).is_some() {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
