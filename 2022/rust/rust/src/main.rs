mod day;

use day::d01::calorie_counting;
use day::d02::rps;
use day::d03::rucksack;
use day::d04::camp_cleanup;
use day::{Input, Output};

fn main() {
    let days: [(fn() -> Input, fn(Input) -> (Output, Output)); 4] = [
        (calorie_counting::parse, calorie_counting::run),
        (rps::parse, rps::run),
        (rucksack::parse, rucksack::run),
        (camp_cleanup::parse, camp_cleanup::run),
    ];

    println!("======");
    for d in days {
        let a = d.0();

        println!("{:?}", d.1(a));
        println!("======");
    }
}
