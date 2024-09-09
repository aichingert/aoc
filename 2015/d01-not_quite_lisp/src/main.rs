use ren_rs::Ren;

fn main() {
    let ren = Ren::init(500, 500, "2015 - day 1");

    for _ in 0..500 {
        ren.draw_frame();
    }
}
