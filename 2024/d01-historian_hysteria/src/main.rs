use ren_rs::{Ren, Vec2};

fn main() {
    let _inp = std::fs::read_to_string("../../input/2024/01").unwrap();
    let mut ren = Ren::new(200, 200, "2024 - day 1");


    loop {
        ren.triangle(Vec2{ x: 0.0, y: -0.5 }, Vec2{ x: 0.5, y: 0.5 }, Vec2{ x: -0.5, y: 0.5 });
        ren.draw();
    }

}
