use crate::day::{Input, InputError, InputResult, Output, Wrapper};
use core::ops::{Add, Mul, Neg, Sub};
use std::collections::HashSet;

const EST: i32 = 350;
pub type Point<T> = (T, T);

fn part_one(points: &mut Vec<Point<i32>>, boarders: &[i32; 4]) -> u32 {
    let (mut area, mut idx): (u32, usize) = (0, 0);

    while idx < points.len() {
        let point = points.remove(idx);

        if let Some(explored) = is_finite(
            &point,
            points,
            boarders,
            &mut HashSet::new(),
            (point.0, point.1),
        ) {
            area = area.max(explored);
        }

        points.insert(idx, point);
        idx += 1;
    }

    area
}

fn is_finite(
    point: &Point<i32>,
    others: &Vec<Point<i32>>,
    boarder: &[i32; 4],
    seen: &mut HashSet<(i32, i32)>,
    cur: (i32, i32),
) -> Option<u32> {
    if seen.contains(&cur) {
        return Some(0);
    }
    seen.insert(cur);

    for i in 0..others.len() {
        let cmp: Point<i32> = (cur.0, cur.1);

        if manhattan_distance(point, &cmp, 0) >= manhattan_distance(&others[i], &cmp, 0) {
            return Some(0);
        }
    }

    let mut area: u32 = 1;

    for cord in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let (x, y) = (cord.0 + cur.0, cord.1 + cur.1);

        if x > boarder[0] || y > boarder[1] || x < boarder[2] || y < boarder[3] {
            return None;
        }

        if let Some(explored) = is_finite(point, others, boarder, seen, (x, y)) {
            area += explored;
        } else {
            return None;
        }
    }

    Some(area)
}

fn part_two(points: &Vec<Point<i32>>, row: Point<i32>, col: Point<i32>) -> u32 {
    let mut ans: u32 = 0;

    for y in row.0..row.1 {
        for x in col.0..col.1 {
            if points
                .iter()
                .map(|p| manhattan_distance(p, &(x, y), 0))
                .sum::<i32>()
                < 10000
            {
                ans += 1;
            }
        }
    }

    ans
}

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
    use crate::day::d06::chronal_coordinates::{manhattan_distance, Point};

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

pub fn run(input: Input) -> (Output, Output) {
    let mut input: (Vec<Point<i32>>, [i32; 4]) = input.unwrap();

    (
        Output::Nu32(part_one(&mut input.0, &input.1)),
        Output::Nu32(part_two(
            &input.0,
            (input.1[3], input.1[1]),
            (input.1[2], input.1[0]),
        )),
    )
}

pub fn parse() -> InputResult<Input> {
    let mut points = Vec::<Point<i32>>::new();
    let mut boarder: [i32; 4] = [0, 0, i32::MAX, i32::MAX];

    for line in std::fs::read_to_string("../input/06")?.lines() {
        points.push(if let Some((x, y)) = line.split_once(", ") {
            (x.parse()?, y.parse()?)
        } else {
            return Err(InputError::InvalidInput);
        });

        boarder[0] = boarder[0].max(points[points.len() - 1].0);
        boarder[2] = boarder[2].min(points[points.len() - 1].0);
        boarder[1] = boarder[1].max(points[points.len() - 1].1);
        boarder[3] = boarder[3].min(points[points.len() - 1].1);
    }

    boarder = [
        boarder[0] + EST,
        boarder[1] + EST,
        boarder[2] - EST,
        boarder[3] - EST,
    ];
    Ok(Input::D06((points, boarder)))
}
