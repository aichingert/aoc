use advent_macros::aoc;

mod Y2015;

#[aoc]
fn main() {
    const YEAR_DAYS: [[u32;5];5] = [[0u32;5]; 5];

    let a = 0;

    Y2015::D01::solve();


    println!("{a}");
}
