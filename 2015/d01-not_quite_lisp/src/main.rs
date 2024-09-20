use ren_rs::Ren;

fn main() {
    //let _inp = std::fs::read_to_string("../../input/2015/01").unwrap();
    
    let mut ren = Ren::new(20, 20, "2015 - day 1");

    for _ in 0..50_000 {
        for k in 0..1_000_000 {
            let x = (k as f32).log(2.0);
            let y = x * 0.5 / 0.1;

            print!("{y}");
        }

        ren.draw();
    }

}
