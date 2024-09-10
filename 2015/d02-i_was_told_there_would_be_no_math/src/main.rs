use ren_rs::Ren;

fn main() {
    let _inp = std::fs::read_to_string("../../input/2015/02").unwrap();

    let ren = Ren::init(500, 500, "2015 - day 2");

    for _ in 0..500 {
        ren.draw_frame();
    }
}
