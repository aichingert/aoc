use ren_rs::{Ren, Vec2};

fn main() {
    let _inp = std::fs::read_to_string("../../input/2024/13").unwrap();
    let mut ren = Ren::new(200, 200, "2024 - day 13");


    loop {
        ren.triangle(Vec2{ x: 0.0, y: -0.5 }, Vec2{ x: 0.5, y: 0.5 }, Vec2{ x: -0.5, y: 0.5 });
        ren.draw();
    }

}
