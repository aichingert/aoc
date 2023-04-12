// Advent of Code 2018, day 6
// (c) aichingert

const ESTIMATION: i32 = 350;

use std::collections::HashSet;

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }

    fn is_closer_to(&self, other: &Self, point: &Self) -> bool {
        self.manhattan_distance(point) < other.manhattan_distance(point)
    }

    fn is_finite(&self, others: &Vec<Self>, borders: &Vec<i32>, seen: &mut HashSet<(i32,i32)>, cur: (i32,i32)) -> (u32, bool) {
        if seen.contains(&cur) {
            return (0, true);
        }
        seen.insert(cur);

        for i in 0..others.len() {
            if !self.is_closer_to(&others[i], &Point::new(cur.0, cur.1)) {
                return (0, true);
            }
        }

        let mut area: u32 = 1;

        for cord in &[(0,1),(1,0),(0,-1),(-1,0)] {
            let (x,y) = (cur.0 + cord.0, cur.1 + cord.1);

            if x < borders[0] || x > borders[1] || y < borders[2] || y > borders[3] {
                return (0, false);
            }

            let result = self.is_finite(others, borders, seen, (x,y));

            if !result.1 {
                return (0, false);
            } else {
                area += result.0;
            }
        }

        (area, true)
    }
}

fn part1(points: &mut Vec<Point>, borders: &Vec<i32>) -> u32 {
    let mut area: u32 = 0;
    let mut i: usize = 0;

    while i < points.len() {
        let current: Point = points.remove(i);

        let result = current.is_finite(&points, borders, &mut HashSet::new(), (current.x, current.y));
        if result.1 {
            area = area.max(result.0);
        } 

        points.insert(i, current);
        i += 1;
    }

    area
}

fn part2(points: &Vec<Point>, row: [i32;2], col: [i32;2]) -> u32 {
    let mut ans = 0;
    
    for y in row[0]..row[1] {
        for x in col[0]..col[1] {
            if points.iter().map(|p| p.manhattan_distance(&Point::new(x,y))).sum::<i32>() < 10000 {
                ans += 1;
            }
        }
    }

    ans
}

fn main() {
    let (mut minx, mut maxx, mut miny, mut maxy) = (i32::MAX, 0, i32::MAX, 0);
    let mut inp = std::fs::read_to_string("../input/06").unwrap()
        .lines()
        .map(|l| {
            let (x,y) = l.split_once(", ").unwrap();
            let (x,y) = (x.parse().unwrap(),y.parse().unwrap());
            minx = minx.min(x);
            maxx = maxx.max(x);
            miny = miny.min(y);
            maxy = maxy.min(y);
            Point::new(x, y)
        }).collect::<Vec<Point>>();

    println!("Part 1: {}", part1(&mut inp, &vec![minx-ESTIMATION,maxx+ESTIMATION,miny-ESTIMATION,maxy+ESTIMATION]));
    println!("Part 2: {}", part2(&inp, [miny-ESTIMATION,maxy+ESTIMATION], [minx-ESTIMATION,maxx+ESTIMATION]));
}
