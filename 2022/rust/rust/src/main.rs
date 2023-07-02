mod day;

use day::d01::calorie_counting;
use day::d02::rps;
use day::{Input, Output};

fn main() {
    let days: [(fn() -> Input, fn(Input) -> (Output, Output)); 2] = [
        (calorie_counting::parse, calorie_counting::run),
        (rps::parse, rps::run),
    ];

    for d in days {
        let a = d.0();

        println!("{:?}", d.1(a));
    }
}
