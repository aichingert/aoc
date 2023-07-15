mod day;

use day::d01::chronal_calibration;
use day::d02::inv_management_sys;
use day::d03::no_matter;
use day::d04::repose_record;
use day::d05::alchemical_reduction;
use day::{Input, InputResult, Output};

fn main() {
    let days: [(fn() -> InputResult<Input>, fn(Input) -> (Output, Output)); 5] = [
        (chronal_calibration::parse, chronal_calibration::run),
        (inv_management_sys::parse, inv_management_sys::run),
        (no_matter::parse, no_matter::run),
        (repose_record::parse, repose_record::run),
        (alchemical_reduction::parse, alchemical_reduction::run),
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
