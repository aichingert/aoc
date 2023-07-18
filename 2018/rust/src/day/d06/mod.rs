pub mod chronal_coordinates;
use core::ops::{Add, Mul, Neg, Sub};

pub type Point<T> = (T, T);

// TODO: move to util mod if I have to use it again -> otherwise the whole
// effort to make this generic would go to waste... still trying to figure
// out how to make the zero dissaper from the parameters
fn manhattan_distance<T>(point: &Point<T>, other: &Point<T>, zero: T) -> T
where
    T: Sub<Output = T> + Add<Output = T> + Mul<Output = T> + Neg<Output = T> + Ord + Copy,
{
    let mut x = point.0 - other.0;
    let mut y = point.1 - other.1;

    x = if x < zero { -x } else { x };
    y = if y < zero { -y } else { y };

    x + y
}

#[cfg(test)]
pub mod test {
    use crate::day::d06::{manhattan_distance, Point};

    #[test]
    fn check_manhattan_normal() {
        let p1: Point<i32> = (10, 10);
        let p2: Point<i32> = (2, 5);

        assert!(manhattan_distance(&p1, &p2, 0i32) == 13);
    }

    #[test]
    fn test_manhattan_negative() {
        let p1: Point<i32> = (10, 10);
        let p2: Point<i32> = (23, 15);

        assert!(manhattan_distance(&p1, &p2, 0i32) == 18);
    }
}
