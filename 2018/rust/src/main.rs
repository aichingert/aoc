mod day;

use day::d01::chronal_calibration;
use day::d02::inv_management_sys;
use day::d03::no_matter;
use day::{Input, InputResult, Output};

fn main() {
    let days: [(fn() -> InputResult<Input>, fn(Input) -> (Output, Output)); 3] = [
        (chronal_calibration::parse, chronal_calibration::run),
        (inv_management_sys::parse, inv_management_sys::run),
        (no_matter::parse, no_matter::run),
    ];

    for day in days {
        match day.0() {
            Ok(input) => {
                println!("{:?}", day.1(input));
            }
            Err(err) => println!("{}", err),
        }
    }
}
