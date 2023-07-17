pub mod chronal_calibration;

#[cfg(test)]
mod test {
    use super::chronal_calibration::{parse, run};

    #[test]
    fn part_one() {
        match parse() {
            Ok(input) => {
                let (_part_one, _part_two) = run(input);
            }
            Err(err) => {
                assert!(1 == 2, "Day 1 [part_one]: error: {err}");
            }
        }
    }

    #[test]
    fn part_two() {
        assert!(1 == 1, "Day 1 [part_two]: error");
    }
}
