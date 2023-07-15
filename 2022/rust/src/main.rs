mod day;

use day::d01::calorie_counting;
use day::d02::rps;
use day::d03::rucksack;
use day::d04::camp_cleanup;
use day::d14::regolith_reservoir;
use day::d15::beacon;
use day::d17::pyroclastic_flow;
use day::d21::monkey_math;
use day::{Input, InputResult, Output};

fn main() {
    let days: [(fn() -> InputResult<Input>, fn(Input) -> (Output, Output)); 8] = [
        (calorie_counting::parse, calorie_counting::run),
        (rps::parse, rps::run),
        (rucksack::parse, rucksack::run),
        (camp_cleanup::parse, camp_cleanup::run),
        (regolith_reservoir::parse, regolith_reservoir::run),
        (beacon::parse, beacon::run),
        (pyroclastic_flow::parse, pyroclastic_flow::run),
        (monkey_math::parse, monkey_math::run),
    ];

    println!("======");
    for d in days {
        match d.0() {
            Ok(input) => println!("{:?}", d.1(input)),
            Err(err) => println!("{}", err),
        }
    }
}
